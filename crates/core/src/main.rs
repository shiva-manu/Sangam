//! Headless CLI entry point for the Sangam runtime.
//!
//! This binary crate is a thin bootstrap shim. Its sole responsibility is to
//! construct the shared state that the runtime needs, wire up operating-system
//! signal handling, and then hand control to [`sangam_core::run`], which
//! contains all of the meaningful async logic.
//!
//! # Shared state
//! All long-lived objects are heap-allocated and wrapped in [`Arc`] so that
//! multiple Tokio tasks can hold a handle to the same instance without
//! copying:
//!
//! | Handle | Type | Purpose |
//! |--------|------|---------|
//! | `shutdown` | `Arc<AtomicBool>` | Signals all tasks to stop gracefully |
//! | `peers` | `Arc<PeerRegistry>` | In-memory set of discovered peers |
//! | `logs` | `Arc<LogBus>` | Fan-out log event broadcaster |
//! | `tasks` | `Arc<TaskTracker>` | Lifecycle tracker for distributed tasks |

use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use sangam_core::logging::LogBus;
use sangam_core::peers::PeerRegistry;
use sangam_core::tasks::tracker::TaskTracker;
use sangam_core::utils::banner::show_banner;
use sangam_core::{install_ctrl_c_handler, run};

#[tokio::main]
async fn main() -> ExitCode {
    // Print the version banner before doing anything else so it is always
    // the first line visible in the terminal, regardless of later log output.
    show_banner();

    // `AtomicBool` used as a cancellation flag. When set to `true` (by the
    // Ctrl-C handler below), every subsystem that holds a clone of this `Arc`
    // will observe the change and begin its shutdown sequence.
    let shutdown = Arc::new(AtomicBool::new(false));

    // Registry of peers discovered via mDNS. Shared between the discovery
    // task (writer) and the networking tasks (readers).
    let peers = Arc::new(PeerRegistry::new());

    // Log event bus. Subsystems publish structured log events here; the TUI
    // (or any other subscriber) receives them via the bus's broadcast channel.
    let logs = Arc::new(LogBus::with_defaults());

    // Tracks every task this node has seen (dispatched or received), storing
    // its current status so the TUI and CLI can report progress.
    let tasks = Arc::new(TaskTracker::new());

    // Register a Ctrl-C (SIGINT) handler that flips `shutdown` to `true`.
    // The handler receives a clone of the Arc so it can signal without
    // needing any other runtime context.
    install_ctrl_c_handler(shutdown.clone());

    // Hand off to the core runtime. `run` owns the event loop and returns
    // only when every subsystem has cleanly stopped (Ok) or when an
    // unrecoverable error forces an early exit (Err).
    match run(shutdown, peers, logs, tasks).await {
        Ok(()) => {
            // Clean shutdown: all tasks finished gracefully.
            println!("[Sangam] Bye.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            // Fatal error: print a human-readable description and exit with a
            // non-zero code so the shell / supervisor knows something went wrong.
            eprintln!("[Sangam] Fatal: {}", e);
            ExitCode::FAILURE
        }
    }
}
