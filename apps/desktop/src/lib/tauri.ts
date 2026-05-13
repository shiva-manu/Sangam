/**
 * @fileoverview Typed wrappers around Tauri's `invoke()` API.
 *
 * Every Rust command name is isolated here so the rest of the app never
 * touches raw command-name strings.  If a Tauri command is renamed or its
 * signature changes, this is the only file that needs updating.
 *
 * All methods on `api` resolve to their Rust return types as declared in
 * `apps/desktop/src-tauri/src/lib.rs`.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  LogEntry,
  MetricsSample,
  NodeInfo,
  Peer,
  StatusCounts,
  TaskRecord,
} from "./types";

/**
 * Typed facade over the Tauri `invoke()` bridge.
 *
 * Method groups:
 *  - **Runtime control** — `startRuntime`, `stopRuntime`: mutate the running
 *    state of the Sangam background runtime.
 *  - **Read-only state** — all remaining methods; safe to poll from React hooks
 *    at any cadence without side effects.
 */
export const api = {
  // ── Runtime control ─────────────────────────────────────────────────────
  /** Start the Sangam background runtime (idempotent if already running). */
  startRuntime: () => invoke<void>("start_runtime"),
  /** Stop the Sangam background runtime and drop all peer connections. */
  stopRuntime: () => invoke<void>("stop_runtime"),

  // ── Read-only state ──────────────────────────────────────────────────────
  /** Retrieve this node's local IP address, port, and running status. */
  getNodeInfo: () => invoke<NodeInfo>("get_node_info"),
  /** Retrieve the list of currently discovered peers. */
  getPeers: () => invoke<Peer[]>("get_peers"),
  /** Retrieve the most recent metrics sample, or `null` if unavailable. */
  getMetrics: () => invoke<MetricsSample | null>("get_metrics"),
  /** Retrieve the full metrics ring-buffer (last N samples at 1 s cadence). */
  getMetricsHistory: () => invoke<MetricsSample[]>("get_metrics_history"),
  /** Retrieve recent log entries from the runtime's in-memory ring buffer. */
  getRecentLogs: () => invoke<LogEntry[]>("get_recent_logs"),
  /** Retrieve all tracked task records within the retention window. */
  getTasks: () => invoke<TaskRecord[]>("get_tasks"),
  /** Retrieve aggregated task counts grouped by status. */
  getTaskStatusCounts: () => invoke<StatusCounts>("get_task_status_counts"),
};

/**
 * Subscribes to the live `sangam:log` Tauri event channel.
 *
 * The Rust bridge in `src-tauri/src/lib.rs::setup` re-emits every
 * `LogBus` entry on this channel.  The returned `UnlistenFn` must be
 * called on component cleanup to prevent listener leaks.
 *
 * @param cb - Callback invoked for each incoming {@link LogEntry}.
 * @returns A promise that resolves to an unlisten function.
 */
export async function onLog(
  cb: (entry: LogEntry) => void,
): Promise<UnlistenFn> {
  return listen<LogEntry>("sangam:log", (event) => cb(event.payload));
}
