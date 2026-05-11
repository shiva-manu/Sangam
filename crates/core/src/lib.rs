//! Sangam — wireless peer-to-peer distributed compute mesh.
//!
//! This library exposes the core building blocks (discovery, networking,
//! models, tasks, utils) so that the `sangam` binary, integration tests,
//! and external consumers (e.g. the Tauri desktop app) can drive the same
//! runtime without duplicating wiring code.

pub mod discovery;
pub mod logging;
pub mod metrics;
pub mod models;
pub mod networking;
pub mod peers;
pub mod tasks;
pub mod utils;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use uuid::Uuid;

use crate::discovery::mdns::start_discovery;
use crate::logging::LogBus;
use crate::networking::server::start_tcp_server;
use crate::peers::PeerRegistry;
use crate::tasks::task::{Task, TaskType};

/// Default TCP port the runtime listens on for peer messages.
pub const DEFAULT_PORT: u16 = 8080;

/// Errors that can prevent the runtime from starting up cleanly.
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("failed to determine local IP address: {0}")]
    NoLocalIp(#[from] local_ip_address::Error),
    #[error("server task failed: {0}")]
    ServerJoin(#[from] tokio::task::JoinError),
}

/// Run the Sangam runtime until `shutdown` is flipped to `true`.
///
/// This is the single entry point used by both the headless `sangam`
/// binary and the Tauri desktop shell. The caller owns the shutdown flag
/// so it can wire it to whatever signal source it prefers (Ctrl-C in the
/// CLI, a "Stop" button in the GUI, etc.).
///
/// The caller also owns the [`PeerRegistry`] so that read-only consumers
/// (Tauri commands, telemetry exporters, future CLI subcommands) can
/// query the live peer list while the runtime drives writes from the
/// discovery loop.
///
/// `logs` is a shared sink for structured runtime events. Pass a fresh
/// `LogBus::with_defaults()` when you don't need to consume the stream
/// (e.g. the headless CLI).
pub async fn run(
    shutdown: Arc<AtomicBool>,
    peers: Arc<PeerRegistry>,
    logs: Arc<LogBus>,
) -> Result<(), RuntimeError> {
    let node_id = Uuid::new_v4().to_string();
    let local_ip = local_ip_address::local_ip()?;
    let port = DEFAULT_PORT;

    logs.info(
        "runtime",
        format!(
            "Sangam runtime starting (node {}, listening on {}:{})",
            short_id(&node_id),
            local_ip,
            port
        ),
    )
    .await;

    let server_handle = tokio::spawn(start_tcp_server(port));

    let demo_task = Task {
        task_id: "demo-sum-001".to_string(),
        task_type: TaskType::Sum,
        numbers: vec![1, 2, 3, 4, 5],
    };

    start_discovery(
        node_id,
        local_ip,
        port,
        shutdown.clone(),
        demo_task,
        peers,
        logs.clone(),
    )
    .await;

    logs.info("runtime", "Sangam runtime shutting down").await;

    server_handle.abort();
    match server_handle.await {
        Ok(()) => Ok(()),
        Err(e) if e.is_cancelled() => Ok(()),
        Err(e) => Err(RuntimeError::ServerJoin(e)),
    }
}

/// First 8 chars of a node UUID — short enough for log lines, still
/// unique enough at typical mesh sizes.
fn short_id(node_id: &str) -> &str {
    node_id.get(..8).unwrap_or(node_id)
}

/// Convenience helper: spawn a Tokio task that flips `shutdown` on Ctrl-C.
///
/// Used by the binary; GUIs typically wire their own stop button instead.
pub fn install_ctrl_c_handler(shutdown: Arc<AtomicBool>) {
    tokio::spawn(async move {
        if let Err(e) = tokio::signal::ctrl_c().await {
            eprintln!("Failed to install Ctrl-C handler: {}", e);
            return;
        }
        println!("\n[Sangam] Received Ctrl-C, shutting down gracefully...");
        shutdown.store(true, Ordering::Relaxed);
    });
}
