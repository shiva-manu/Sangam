/**
 * @fileoverview Runtime data hooks consumed by dashboard panels.
 *
 * This module exposes one hook per Tauri data source so components can opt into
 * exactly the streams they need.  Each hook owns its own polling cadence and
 * initial value, keeping page components thin and data-fetching concerns local.
 */
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

// Fast poll: status + metrics drive the top bar and charts, so 1 s keeps the
// UI feeling live without overloading the local Tauri bridge.
const POLL_FAST = 1000;
// Medium poll: peers and task rows change less frequently and tolerate a small
// delay, so 2 s reduces churn in animated lists.
const POLL_MED = 2000;
// Slow poll: aggregate counts are cheap but visually low-priority; 4 s is
// enough to keep badges current without duplicating task-list work.
const POLL_SLOW = 4000;

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
// Logs are special: unlike polled resources, the console needs history and a
// live stream.  First we backfill from the Rust ring buffer so the console is
// useful immediately, then we tail the `sangam:log` event channel.
// LOG_BUFFER_CAP bounds browser-side memory so the console cannot grow forever.
// ---------------------------------------------------------------------------

const LOG_BUFFER_CAP = 500;

export function useLogs() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  // Ref-mirror avoids stale closures in the event listener: the callback can
  // append to the latest log array without depending on React state captured
  // when the listener was first registered.
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
        // Drop oldest entries past LOG_BUFFER_CAP. Rust keeps a shorter ring
        // buffer; the UI keeps extra scrollback while still preventing
        // unbounded growth during long-running sessions.
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
