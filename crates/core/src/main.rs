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
    show_banner();

    let shutdown = Arc::new(AtomicBool::new(false));
    let peers = Arc::new(PeerRegistry::new());
    let logs = Arc::new(LogBus::with_defaults());
    let tasks = Arc::new(TaskTracker::new());
    install_ctrl_c_handler(shutdown.clone());

    match run(shutdown, peers, logs, tasks).await {
        Ok(()) => {
            println!("[Sangam] Bye.");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[Sangam] Fatal: {}", e);
            ExitCode::FAILURE
        }
    }
}
