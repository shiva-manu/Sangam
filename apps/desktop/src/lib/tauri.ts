// Typed wrappers around `invoke()` so the rest of the app never touches
// raw Tauri command names. If a Rust command renames, this file is the
// only place that has to change.

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

export const api = {
  // Runtime control.
  startRuntime: () => invoke<void>("start_runtime"),
  stopRuntime: () => invoke<void>("stop_runtime"),

  // Read-only state.
  getNodeInfo: () => invoke<NodeInfo>("get_node_info"),
  getPeers: () => invoke<Peer[]>("get_peers"),
  getMetrics: () => invoke<MetricsSample | null>("get_metrics"),
  getMetricsHistory: () => invoke<MetricsSample[]>("get_metrics_history"),
  getRecentLogs: () => invoke<LogEntry[]>("get_recent_logs"),
  getTasks: () => invoke<TaskRecord[]>("get_tasks"),
  getTaskStatusCounts: () => invoke<StatusCounts>("get_task_status_counts"),
};

/// Subscribe to the live `sangam:log` event. The bridge in
/// `src-tauri/src/lib.rs::setup` re-emits every LogBus entry on this
/// channel — we just decode and forward.
export async function onLog(
  cb: (entry: LogEntry) => void,
): Promise<UnlistenFn> {
  return listen<LogEntry>("sangam:log", (event) => cb(event.payload));
}
