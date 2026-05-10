use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use Sangam::discovery::mdns::start_discovery;
use Sangam::networking::server::start_tcp_server;
use Sangam::tasks::task::{Task, TaskType};
use Sangam::utils::banner::show_banner;

use uuid::Uuid;

#[tokio::main]
async fn main() -> ExitCode {
    show_banner();

    let node_id = Uuid::new_v4().to_string();

    let local_ip = match local_ip_address::local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Failed to determine local IP address: {}", e);
            eprintln!(
                "Sangam needs an active network interface (WiFi or Ethernet) to advertise itself."
            );
            return ExitCode::FAILURE;
        }
    };

    let port: u16 = 8080;

    // Shared shutdown flag so a Ctrl-C can stop discovery gracefully and
    // give the mDNS daemon time to send Goodbye packets.
    let shutdown = Arc::new(AtomicBool::new(false));

    // Forward Ctrl-C to the shutdown flag without blocking the main task.
    let shutdown_signal = shutdown.clone();
    tokio::spawn(async move {
        if let Err(e) = tokio::signal::ctrl_c().await {
            eprintln!("Failed to install Ctrl-C handler: {}", e);
            return;
        }
        println!("\n[Sangam] Received Ctrl-C, shutting down gracefully...");
        shutdown_signal.store(true, Ordering::Relaxed);
    });

    // Hold on to the server's JoinHandle so a panic in the accept loop
    // surfaces instead of being silently swallowed.
    let server_handle = tokio::spawn(start_tcp_server(port));

    // Create a demo task to distribute to discovered peers.
    let demo_task = Task {
        task_id: "demo-sum-001".to_string(),
        task_type: TaskType::Sum,
        numbers: vec![1, 2, 3, 4, 5],
    };

    // Run mDNS discovery until the shutdown flag is flipped.
    start_discovery(node_id, local_ip, port, shutdown.clone(), demo_task).await;

    // Tear down the (otherwise-infinite) server task. abort() is fine here
    // because every connection handler is independently spawned; the only
    // thing we cancel is the accept loop itself.
    server_handle.abort();
    if let Err(e) = server_handle.await
        && !e.is_cancelled()
    {
        eprintln!("[Sangam] TCP server task ended unexpectedly: {}", e);
    }

    println!("[Sangam] Bye.");
    ExitCode::SUCCESS
}
