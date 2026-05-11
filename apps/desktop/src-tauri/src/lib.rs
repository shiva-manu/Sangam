//! Tauri command layer for the Sangam desktop app.
//!
//! This crate is **strictly a thin shell** around `sangam-core`. The rule:
//! every command here is a one-liner that delegates to the runtime. If a
//! command grows logic, that logic belongs in `sangam-core`, not here.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use sangam_core::metrics::{
    DEFAULT_HISTORY_CAPACITY, DEFAULT_INTERVAL, MetricsSample, MetricsStore, run_collector,
};
use sangam_core::peers::{Peer, PeerRegistry};
use sangam_core::{DEFAULT_PORT, run as run_runtime};
use serde::Serialize;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

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
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            handle: Mutex::new(None),
            shutdown: Mutex::new(None),
            peers: Arc::new(PeerRegistry::new()),
            metrics: Arc::new(MetricsStore::new(DEFAULT_HISTORY_CAPACITY)),
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

    // Hand the runtime our shared registry so the UI sees peers as they
    // arrive without needing to plumb a separate channel.
    let peers = state.peers.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = run_runtime(shutdown, peers).await {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
