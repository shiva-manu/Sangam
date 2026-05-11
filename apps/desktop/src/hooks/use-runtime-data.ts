import { useEffect, useRef, useState } from "react";
import { api, onLog } from "../lib/tauri";
import type {
  LogEntry,
  MetricsSample,
  NodeInfo,
  Peer,
  StatusCounts,
  TaskRecord,
} from "../lib/types";
import { usePolled } from "./use-polled";

const POLL_FAST = 1000; // node info / metrics — feel snappy
const POLL_MED = 2000; // peers / tasks
const POLL_SLOW = 4000; // task counts (cheap aggregate)

// ---------------------------------------------------------------------------
// One-resource-per-hook so individual panels can opt in. Every hook owns
// its own polling cadence — the dashboard usually subscribes to several.
// ---------------------------------------------------------------------------

export function useNodeInfo() {
  const init: NodeInfo = { local_ip: "—", port: 0, running: false };
  return usePolled<NodeInfo>(api.getNodeInfo, POLL_FAST, init);
}

export function usePeers() {
  return usePolled<Peer[]>(api.getPeers, POLL_MED, []);
}

export function useMetrics() {
  return usePolled<MetricsSample | null>(api.getMetrics, POLL_FAST, null);
}

export function useMetricsHistory() {
  return usePolled<MetricsSample[]>(api.getMetricsHistory, POLL_FAST, []);
}

export function useTasks() {
  return usePolled<TaskRecord[]>(api.getTasks, POLL_MED, []);
}

export function useStatusCounts() {
  const init: StatusCounts = {
    queued: 0,
    running: 0,
    completed: 0,
    failed: 0,
  };
  return usePolled<StatusCounts>(api.getTaskStatusCounts, POLL_SLOW, init);
}

// ---------------------------------------------------------------------------
// Logs are special: they have a backfill phase (load last N from the
// ring buffer) and then a tail phase (subscribe to `sangam:log` events).
// We cap the in-memory buffer so the console doesn't grow forever.
// ---------------------------------------------------------------------------

const LOG_BUFFER_CAP = 500;

export function useLogs() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  // Use a ref-mirror to avoid stale-closure issues in the listener callback.
  const logsRef = useRef<LogEntry[]>([]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    let cancelled = false;

    void (async () => {
      // Backfill from the ring buffer first so the console isn't blank.
      try {
        const recent = await api.getRecentLogs();
        if (!cancelled) {
          setLogs(recent);
          logsRef.current = recent;
        }
      } catch {
        /* ignore — runtime might not be ready yet */
      }
      // Then attach the live tail.
      unlisten = await onLog((entry) => {
        const next = [...logsRef.current, entry];
        // Drop the oldest entries if we've exceeded the cap. The ring
        // buffer in Rust caps at 200; here we keep more so user-side
        // scrolling has headroom.
        if (next.length > LOG_BUFFER_CAP) {
          next.splice(0, next.length - LOG_BUFFER_CAP);
        }
        logsRef.current = next;
        setLogs(next);
      });
    })();

    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, []);

  return logs;
}
