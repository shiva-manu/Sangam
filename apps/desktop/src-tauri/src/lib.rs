//! Tauri command layer for the Sangam desktop app.
//!
//! This crate is **strictly a thin shell** around `sangam-core`. The rule:
//! every command here is a one-liner that delegates to the runtime. If a
//! command grows logic, that logic belongs in `sangam-core`, not here.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use sangam_core::logging::{LogBus, LogEntry};
use sangam_core::metrics::{
    DEFAULT_HISTORY_CAPACITY, DEFAULT_INTERVAL, MetricsSample, MetricsStore, run_collector,
};
use sangam_core::peers::{Peer, PeerRegistry};
use sangam_core::tasks::tracker::{StatusCounts, TaskRecord, TaskTracker};
use sangam_core::{DEFAULT_PORT, run as run_runtime};
use serde::Serialize;
use tauri::{Emitter, Manager, State};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Tauri event channel name the frontend subscribes to via `listen()`.
const LOG_EVENT: &str = "sangam:log";

/// Shared state Tauri keeps alive for the whole app lifetime.
///
/// We hold the runtime's `JoinHandle` so the UI can stop the engine via
/// the `stop_runtime` command, the shutdown flag so it can do so
/// gracefully (instead of an `abort()`), and the peer registry so
/// read-only commands like `get_peers` can serve data without needing
/// to bounce through the runtime task.
struct RuntimeState {
    handle: Mutex<Option<JoinHandle<()>>>,
    shutdown: Mutex<Option<Arc<AtomicBool>>>,
    peers: Arc<PeerRegistry>,
    metrics: Arc<MetricsStore>,
    logs: Arc<LogBus>,
    tasks: Arc<TaskTracker>,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            handle: Mutex::new(None),
            shutdown: Mutex::new(None),
            peers: Arc::new(PeerRegistry::new()),
            metrics: Arc::new(MetricsStore::new(DEFAULT_HISTORY_CAPACITY)),
            logs: Arc::new(LogBus::with_defaults()),
            tasks: Arc::new(TaskTracker::new()),
        }
    }
}

/// Snapshot of node info the UI renders in the dashboard header.
#[derive(Serialize)]
struct NodeInfo {
    local_ip: String,
    port: u16,
    running: bool,
}

/// Start the Sangam runtime in a background Tokio task.
///
/// Returns immediately so the UI stays responsive. Subsequent calls
/// while a runtime is already running are no-ops.
#[tauri::command]
async fn start_runtime(state: State<'_, RuntimeState>) -> Result<(), String> {
    let mut handle_slot = state.handle.lock().await;
    if handle_slot.as_ref().is_some_and(|h| !h.is_finished()) {
        return Ok(()); // already running — idempotent
    }

    let shutdown = Arc::new(AtomicBool::new(false));
    *state.shutdown.lock().await = Some(shutdown.clone());

    // Hand the runtime our shared registry, log bus, and task tracker
    // so the UI sees peers, structured events, and task lifecycles as
    // they arrive without needing to plumb a separate channel for each.
    let peers = state.peers.clone();
    let logs = state.logs.clone();
    let tasks = state.tasks.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = run_runtime(shutdown, peers, logs, tasks).await {
            eprintln!("[Sangam] runtime exited with error: {}", e);
        }
    });
    *handle_slot = Some(handle);
    Ok(())
}

/// Flip the shutdown flag and await runtime teardown.
///
/// Safe to call when the runtime is already stopped.
#[tauri::command]
async fn stop_runtime(state: State<'_, RuntimeState>) -> Result<(), String> {
    if let Some(flag) = state.shutdown.lock().await.take() {
        flag.store(true, Ordering::Relaxed);
    }
    if let Some(handle) = state.handle.lock().await.take()
        && let Err(e) = handle.await
        && !e.is_cancelled()
    {
        return Err(format!("runtime task panicked: {}", e));
    }
    Ok(())
}

/// Read-only snapshot for the dashboard.
#[tauri::command]
async fn get_node_info(state: State<'_, RuntimeState>) -> Result<NodeInfo, String> {
    let local_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "unavailable".to_string());

    let running = state
        .handle
        .lock()
        .await
        .as_ref()
        .is_some_and(|h| !h.is_finished());

    Ok(NodeInfo {
        local_ip,
        port: DEFAULT_PORT,
        running,
    })
}

/// Snapshot of all peers known to the registry, freshest first.
///
/// Safe to poll: the registry is in-memory and reads are cheap. The UI
/// hits this on a 2s interval to drive the live mesh visualisation and
/// node-list cards.
#[tauri::command]
async fn get_peers(state: State<'_, RuntimeState>) -> Result<Vec<Peer>, String> {
    Ok(state.peers.list().await)
}

/// Latest local resource sample (CPU / RAM / network).
///
/// Returns `None` until the collector has produced its first sample
/// (~1 interval after app boot).
#[tauri::command]
async fn get_metrics(state: State<'_, RuntimeState>) -> Result<Option<MetricsSample>, String> {
    Ok(state.metrics.latest().await)
}

/// Recent metrics history, oldest first. Drives the dashboard charts.
#[tauri::command]
async fn get_metrics_history(state: State<'_, RuntimeState>) -> Result<Vec<MetricsSample>, String> {
    Ok(state.metrics.history().await)
}

/// Backfill of recent log entries — used when the runtime console mounts
/// so the user sees context, not an empty pane. Live updates after that
/// arrive via the `sangam:log` Tauri event.
#[tauri::command]
async fn get_recent_logs(state: State<'_, RuntimeState>) -> Result<Vec<LogEntry>, String> {
    Ok(state.logs.recent().await)
}

/// All tracked tasks (inbound + outbound), most recent first. Drives
/// the dashboard's task list.
#[tauri::command]
async fn get_tasks(state: State<'_, RuntimeState>) -> Result<Vec<TaskRecord>, String> {
    Ok(state.tasks.list().await)
}

/// Per-status histogram of all tracked tasks. Drives the dashboard's
/// status pill counters (queued / running / completed / failed).
#[tauri::command]
async fn get_task_status_counts(state: State<'_, RuntimeState>) -> Result<StatusCounts, String> {
    Ok(state.tasks.status_counts().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let state = RuntimeState::default();

            // The metrics collector runs for the entire app lifetime so
            // the dashboard always has fresh local CPU/RAM/network data,
            // even when the user has the runtime stopped. We never need
            // to stop it before exit because Tauri tears down the tokio
            // runtime when the window closes.
            let metrics = state.metrics.clone();
            let collector_shutdown = Arc::new(AtomicBool::new(false));
            tauri::async_runtime::spawn(async move {
                run_collector(metrics, collector_shutdown, DEFAULT_INTERVAL).await;
            });

            // Log bridge: subscribe to the bus *before* anyone starts
            // emitting (i.e. before run_runtime is ever called) so we
            // don't drop early entries. Each LogEntry gets emitted to
            // the frontend as a `sangam:log` event; the UI's listen()
            // call renders it into the runtime console.
            let mut log_rx = state.logs.subscribe();
            let log_app = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use tokio::sync::broadcast::error::RecvError;
                loop {
                    match log_rx.recv().await {
                        Ok(entry) => {
                            // Emit failures (no listeners yet, etc.) are
                            // expected at boot — silent.
                            let _ = log_app.emit(LOG_EVENT, entry);
                        }
                        Err(RecvError::Lagged(_)) => {
                            // Slow consumer — skip the gap. The frontend
                            // can re-sync via get_recent_logs.
                            continue;
                        }
                        Err(RecvError::Closed) => break,
                    }
                }
            });

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_runtime,
            stop_runtime,
            get_node_info,
            get_peers,
            get_metrics,
            get_metrics_history,
            get_recent_logs,
            get_tasks,
            get_task_status_counts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
