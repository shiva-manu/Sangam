use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::models::message::{MessageType, NodeMessage};
use crate::tasks::task::Task;
use crate::tasks::tracker::TaskTracker;

/// How long to wait for a peer's response before giving up.
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(10);

/// Demo entry point used by mDNS discovery: connect to a peer and send the
/// provided task, then print whatever it responds.
///
/// The tracker is updated through the full lifecycle:
///   * `record_outbound` (Queued) before the connection attempt
///   * `mark_running`             once the request is on the wire
///   * `mark_completed` / `mark_failed` once a response (or error) arrives
pub async fn connect_to_node(
    target: String,
    node_id: String,
    task: Task,
    peer_id: Option<String>,
    tracker: Arc<TaskTracker>,
) {
    let task_id = task.task_id.clone();
    let task_type_label = format!("{:?}", task.task_type);

    tracker
        .record_outbound(&task_id, &task_type_label, peer_id)
        .await;

    println!("Connecting to {}\n", target);

    let stream = match TcpStream::connect(&target).await {
        Ok(s) => s,
        Err(e) => {
            println!("Connection failed: {}\n", e);
            tracker
                .mark_failed(&task_id, format!("connect failed: {}", e))
                .await;
            return;
        }
    };

    println!("Connected to {}\n", target);

    let message = NodeMessage {
        node_id,
        message_type: MessageType::Task(task),
    };

    // Once we hand the request off to send_message, the request is in
    // flight on the wire — mark the lifecycle accordingly.
    tracker.mark_running(&task_id).await;

    match send_message(stream, &message).await {
        Ok(response) => {
            println!("Received Response:");
            match serde_json::to_string_pretty(&response) {
                Ok(j) => println!("{}\n", j),
                Err(e) => {
                    eprintln!("Failed to serialize response for display: {}\n", e);
                    println!("{:?}\n", response);
                }
            }
            // Update the tracker based on what the peer sent back.
            match response.message_type {
                MessageType::Result(result) => {
                    if result.status == "completed" {
                        tracker.mark_completed(&task_id, result.result.into()).await;
                    } else {
                        tracker
                            .mark_failed(&task_id, format!("worker status: {}", result.status))
                            .await;
                    }
                }
                other => {
                    tracker
                        .mark_failed(
                            &task_id,
                            format!("unexpected response variant: {:?}", other),
                        )
                        .await;
                }
            }
        }
        Err(e) => {
            println!("Failed to communicate with peer: {}\n", e);
            tracker.mark_failed(&task_id, e.to_string()).await;
        }
    }
}

/// Send a single newline-delimited `NodeMessage` over `stream` and wait up
/// to [`RESPONSE_TIMEOUT`] for one newline-delimited `NodeMessage` response.
///
/// Returns an error if the write fails, the response can't be parsed, the
/// peer hangs up before responding, or the timeout elapses. Extracted from
/// [`connect_to_node`] so it can be unit-tested directly.
pub async fn send_message(
    stream: TcpStream,
    message: &NodeMessage,
) -> std::io::Result<NodeMessage> {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let json = serde_json::to_string(message)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    write_half.write_all(json.as_bytes()).await?;
    write_half.write_all(b"\n").await?;
    write_half.flush().await?;

    let mut line = String::new();
    let bytes_read = match timeout(RESPONSE_TIMEOUT, reader.read_line(&mut line)).await {
        Ok(res) => res?,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "timed out waiting for peer response",
            ));
        }
    };

    if bytes_read == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "peer closed connection without responding",
        ));
    }

    let response: NodeMessage = serde_json::from_str(line.trim_end())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(response)
}
