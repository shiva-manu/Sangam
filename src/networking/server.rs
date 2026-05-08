use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::models::message::{MessageType, NodeMessage};
use crate::tasks::executor::execute_task;

pub async fn start_tcp_server(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to start TCP server");
    println!("TCP Server Running on port {}\n", port);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        println!("Incoming connection from {}\n", addr);

        tokio::spawn(async move {
            let mut buffer = vec![0; 4096];

            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let received = String::from_utf8_lossy(&buffer[..n]);

                    println!("Raw Message:");
                    println!("{}", received);

                    let parsed: Result<NodeMessage, _> = serde_json::from_str(&received);

                    match parsed {
                        Ok(message) => match message.message_type {
                            MessageType::Ping => {
                                println!("Ping Received");
                            }

                            MessageType::Task(task) => {
                                println!("\nExecuting Task: {:?}\n", task);

                                let result = execute_task(task).await;

                                let response = NodeMessage {
                                    node_id: message.node_id.clone(),
                                    message_type: MessageType::Result(result),
                                };

                                let json = serde_json::to_string(&response).unwrap();

                                socket.write_all(json.as_bytes()).await.unwrap();

                                println!("Result Sent Back\n");
                            }

                            MessageType::Result(result) => {
                                println!("Task Result: {:?}", result);
                            }
                        },
                        Err(e) => {
                            println!("Failed to parse message: {}", e);
                        }
                    }
                                }
                _ => {}
            }
        });
    }
}
