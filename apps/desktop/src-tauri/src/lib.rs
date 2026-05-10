//! Tauri command layer for the Sangam desktop app.
//!
//! This crate is **strictly a thin shell** around `sangam-core`. The rule:
//! every command here is a one-liner that delegates to the runtime. If a
//! command grows logic, that logic belongs in `sangam-core`, not here.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use sangam_core::{DEFAULT_PORT, run as run_runtime};
use serde::Serialize;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Shared state Tauri keeps alive for the whole app lifetime.
///
/// We hold the runtime's `JoinHandle` so the UI can stop the engine via
/// the `stop_runtime` command, and the shutdown flag so it can do so
/// gracefully (instead of an abort()).
#[derive(Default)]
struct RuntimeState {
    handle: Mutex<Option<JoinHandle<()>>>,
    shutdown: Mutex<Option<Arc<AtomicBool>>>,
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

    let handle = tokio::spawn(async move {
        if let Err(e) = run_runtime(shutdown).await {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(RuntimeState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_runtime,
            stop_runtime,
            get_node_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
