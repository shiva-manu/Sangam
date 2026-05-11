use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::models::message::{MessageType, NodeMessage};
use crate::tasks::executor::execute_task;
use crate::tasks::tracker::TaskTracker;

/// Bind a TCP listener and spawn a connection handler per accepted socket.
/// This function never returns under normal operation.
pub async fn start_tcp_server(port: u16, tracker: Arc<TaskTracker>) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to start TCP server");
    println!("TCP Server Running on port {}\n", port);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Incoming connection from {}\n", addr);
                let t = tracker.clone();
                tokio::spawn(async move {
                    handle_connection(socket, t).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

/// Handle a single inbound connection: read one newline-delimited
/// `NodeMessage`, dispatch on its variant, and (for `Task`) write a
/// newline-delimited `Result` response back. Extracted from
/// `start_tcp_server` so it can be exercised by integration tests.
pub async fn handle_connection(socket: TcpStream, tracker: Arc<TaskTracker>) {
    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);
    let mut line = String::new();

    let bytes_read = match reader.read_line(&mut line).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            return;
        }
    };

    if bytes_read == 0 {
        return; // peer closed without sending anything
    }

    let raw = line.trim_end();
    println!("Raw Message:");
    println!("{}", raw);

    let message: NodeMessage = match serde_json::from_str(raw) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
            return;
        }
    };

    match message.message_type {
        MessageType::Ping => {
            println!("Ping Received");

            let response = NodeMessage {
                node_id: message.node_id.clone(),
                message_type: MessageType::Pong,
            };

            let json = match serde_json::to_string(&response) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Failed to serialize Pong response: {}", e);
                    return;
                }
            };

            if let Err(e) = write_half.write_all(json.as_bytes()).await {
                eprintln!("Failed to send Pong response: {}", e);
                return;
            }
            if let Err(e) = write_half.write_all(b"\n").await {
                eprintln!("Failed to send Pong delimiter: {}", e);
                return;
            }
            if let Err(e) = write_half.flush().await {
                eprintln!("Failed to flush Pong response: {}", e);
                return;
            }

            println!("Pong Sent Back\n");
        }

        MessageType::Pong => {
            println!("Pong Received (ping acknowledged)");
        }

        MessageType::Task(task) => {
            println!("\nExecuting Task: {:?}\n", task);

            // Inbound lifecycle: record on receipt (Queued), advance to
            // Running before the executor call, then Completed/Failed
            // based on what the executor reports.
            let task_id = task.task_id.clone();
            let task_type_label = format!("{:?}", task.task_type);
            let owner = Some(message.node_id.clone());
            tracker
                .record_inbound(&task_id, &task_type_label, owner)
                .await;
            tracker.mark_running(&task_id).await;

            let result = execute_task(task);

            // Reflect the executor's verdict in the tracker before we
            // send anything on the wire — guarantees the dashboard sees
            // a consistent state even if the response write fails.
            if result.status == "completed" {
                tracker.mark_completed(&task_id, result.result.into()).await;
            } else {
                tracker
                    .mark_failed(&task_id, format!("executor status: {}", result.status))
                    .await;
            }

            let response = NodeMessage {
                node_id: message.node_id.clone(),
                message_type: MessageType::Result(result),
            };

            let json = match serde_json::to_string(&response) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Failed to serialize response: {}", e);
                    return;
                }
            };

            if let Err(e) = write_half.write_all(json.as_bytes()).await {
                eprintln!("Failed to send response: {}", e);
                return;
            }
            if let Err(e) = write_half.write_all(b"\n").await {
                eprintln!("Failed to send response delimiter: {}", e);
                return;
            }
            if let Err(e) = write_half.flush().await {
                eprintln!("Failed to flush response: {}", e);
                return;
            }

            println!("Result Sent Back\n");
        }

        MessageType::Result(result) => {
            println!("Task Result: {:?}", result);
        }
    }
}
