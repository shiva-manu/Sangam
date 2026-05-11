// TypeScript mirror of the Rust serde shapes from `sangam-core` and the
// Tauri command layer in `apps/desktop/src-tauri/src/lib.rs`.
//
// Keep these in sync by hand. If a Rust struct changes its serde repr,
// the matching TS type has to change too — the `cargo test` suite has
// JSON-shape tests that lock the Rust side; this file is the front-of-
// the-house contract.

export type NodeInfo = {
  local_ip: string;
  port: number;
  running: boolean;
};

export type Peer = {
  id: string;
  name: string;
  /// `addr` is rendered by Rust's `SocketAddr::Display` — e.g. `192.168.1.4:8080`.
  addr: string;
  /// Last time we saw this peer (Unix ms).
  last_seen_ms: number;
  cpu_threads?: number;
  ram_gib?: number;
};

export type MetricsSample = {
  timestamp_ms: number;
  cpu_pct: number;
  ram_used_mb: number;
  ram_total_mb: number;
  net_rx_kbps: number;
  net_tx_kbps: number;
};

export type LogLevel = "debug" | "info" | "warn" | "error";

export type LogEntry = {
  timestamp_ms: number;
  level: LogLevel;
  /// "discovery" | "networking" | "tasks" | "runtime" — free-form.
  source: string;
  message: string;
};

export type TaskStatus = "queued" | "running" | "completed" | "failed";
export type TaskDirection = "outbound" | "inbound";

export type TaskRecord = {
  task_id: string;
  task_type: string;
  direction: TaskDirection;
  status: TaskStatus;
  peer_id?: string;
  created_at_ms: number;
  started_at_ms?: number;
  completed_at_ms?: number;
  result?: number;
  error?: string;
};

export type StatusCounts = {
  queued: number;
  running: number;
  completed: number;
  failed: number;
};
