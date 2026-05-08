use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::models::message::{MessageType, NodeMessage};
use crate::tasks::task::{Task, TaskType};

/// How long to wait for a peer's response before giving up.
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(10);

/// Demo entry point used by mDNS discovery: connect to a peer and send a
/// hard-coded `Sum` task, then print whatever it responds.
pub async fn connect_to_node(target: String, node_id: String) {
    println!("Connecting to {}\n", target);

    let stream = match TcpStream::connect(&target).await {
        Ok(s) => s,
        Err(e) => {
            println!("Connection failed: {}\n", e);
            return;
        }
    };

    println!("Connected to {}\n", target);

    let task = Task {
        task_id: "task-001".to_string(),
        task_type: TaskType::Sum,
        numbers: vec![1, 2, 3, 4, 5],
    };

    let message = NodeMessage {
        node_id,
        message_type: MessageType::Task(task),
    };

    match send_message(stream, &message).await {
        Ok(response) => {
            println!("Received Response:");
            match serde_json::to_string(&response) {
                Ok(j) => println!("{}\n", j),
                Err(_) => println!("{:?}\n", response),
            }
        }
        Err(e) => {
            println!("Failed to communicate with peer: {}\n", e);
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
            ))
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