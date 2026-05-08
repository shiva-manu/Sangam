use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::models::message::{MessageType, NodeMessage};
use crate::tasks::task::{Task, TaskType};

pub async fn connect_to_node(target: String, node_id: String) {
    println!("Connecting to {}\n", target);

    match TcpStream::connect(&target).await {
        Ok(mut stream) => {
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

            let json = serde_json::to_string(&message).unwrap();

            stream.write_all(json.as_bytes()).await.unwrap();

            println!("Task Sent Successfully\n");

            let mut buffer = vec![0; 4096];

            match stream.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let response = String::from_utf8_lossy(&buffer[..n]);

                    println!("Received Response:");
                    println!("{}", response);
                    println!();
                }

                _ => {}
            }
        }

        Err(e) => {
            println!("Connection failed: {}\n", e);
        }
    }
}