/**
 * @fileoverview TypeScript mirror of the Rust `serde` shapes from `sangam-core`
 * and the Tauri command layer (`apps/desktop/src-tauri/src/lib.rs`).
 *
 * These types must be kept in sync with their Rust counterparts by hand.
 * If a Rust struct changes its `serde` representation the matching TS type
 * must change too.  The `cargo test` suite has JSON-shape tests that lock the
 * Rust side; this file is the front-of-the-house contract.
 *
 * Field naming follows Rust's `serde(rename_all = "snake_case")` convention.
 */

/** Mirrors `NodeInfo` in `src-tauri/src/lib.rs`. Returned by `get_node_info`. */
export type NodeInfo = {
  // The local IP address this node is bound to (e.g. `"192.168.1.10"`).
  local_ip: string;
  // The UDP/TCP port the Sangam runtime is listening on.
  port: number;
  // `true` when the Sangam runtime is actively running and accepting tasks.
  running: boolean;
};

/** Mirrors `Peer` in `sangam-core`. One entry per discovered mDNS neighbour. */
export type Peer = {
  // Stable identifier assigned by the runtime — the primary key for this peer everywhere.
  id: string;
  // mDNS service name, e.g. `"lab-mbp-01._sangam._udp.local."`.
  name: string;
  // Rendered by Rust's `SocketAddr::Display` — e.g. `"192.168.1.4:8080"`.
  addr: string;
  // Unix epoch milliseconds of the most recent mDNS heartbeat from this peer.
  last_seen_ms: number;
  // Advertised via mDNS TXT record; absent if the peer has not published it.
  cpu_threads?: number;
  // Advertised via mDNS TXT record; absent if the peer has not published it.
  ram_gib?: number;
};

/** Mirrors `MetricsSample` in `sangam-core`. One sample per metrics tick (≈1 s). */
export type MetricsSample = {
  // Unix epoch milliseconds when this sample was captured.
  timestamp_ms: number;
  // System-wide CPU utilisation as a percentage (0–100).
  cpu_pct: number;
  // RAM currently in use, in megabytes.
  ram_used_mb: number;
  // Total physical RAM available, in megabytes.
  ram_total_mb: number;
  // Network receive throughput in KB/s.
  net_rx_kbps: number;
  // Network transmit throughput in KB/s.
  net_tx_kbps: number;
};

/** Mirrors the `LogLevel` enum in `sangam-core`. Maps to standard severity levels. */
export type LogLevel = "debug" | "info" | "warn" | "error";

/** Mirrors `LogEntry` in `sangam-core`. Emitted on the `sangam:log` Tauri event channel. */
export type LogEntry = {
  // Unix epoch milliseconds when this entry was created by the runtime.
  timestamp_ms: number;
  // Severity level — used to colour-code rows in the Runtime Console.
  level: LogLevel;
  // Free-form subsystem tag — e.g. `"discovery"`, `"networking"`, `"tasks"`, `"runtime"`.
  source: string;
  // Human-readable log message emitted by the runtime.
  message: string;
};

/**
 * Valid states in the task lifecycle state machine.
 * Transitions follow: queued → running → completed | failed.
 * The terminal states (`completed`, `failed`) are never exited.
 */
export type TaskStatus = "queued" | "running" | "completed" | "failed";
/** Whether this task was dispatched to a peer (`outbound`) or received from one (`inbound`). */
export type TaskDirection = "outbound" | "inbound";

/** Mirrors `TaskRecord` in `sangam-core`. One entry per dispatched or received task. */
export type TaskRecord = {
  // Unique task identifier assigned by the runtime.
  task_id: string;
  // Application-level task descriptor used to label the work being executed.
  task_type: string;
  // Whether this task was sent to a peer or received from one.
  direction: TaskDirection;
  // Current position in the task state machine (see `TaskStatus`).
  status: TaskStatus;
  // The peer on the other end of this task; absent for local-only tasks.
  peer_id?: string;
  // Unix ms when the task was first enqueued.
  created_at_ms: number;
  // Unix ms when execution began; absent until the task transitions to `running`.
  started_at_ms?: number;
  // Unix ms when the task reached a terminal state (`completed` or `failed`).
  completed_at_ms?: number;
  // Numeric return value; present only when `status === "completed"`.
  result?: number;
  // Error message; present only when `status === "failed"`.
  error?: string;
};

/** Mirrors `StatusCounts` in `sangam-core`. Aggregate task counts by lifecycle state. */
export type StatusCounts = {
  // Number of tasks waiting to be picked up by a worker.
  queued: number;
  // Number of tasks currently being executed.
  running: number;
  // Number of tasks currently tracked in the successful terminal state.
  completed: number;
  // Number of tasks currently tracked in the failed terminal state.
  failed: number;
};
