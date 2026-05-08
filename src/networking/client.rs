use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::models::message::NodeMessage;

pub async fn connect_to_node(target: String, node_id: String) {
    println!("Connecting to {}\n", target);

    match TcpStream::connect(&target).await {
        Ok(mut stream) => {
            println!("Connected to {}\n", target);

            let message = NodeMessage {
                node_id,
                message: "Hello from Sangam Node".to_string(),
            };
            let json = serde_json::to_string(&message).unwrap();
            stream.write_all(json.as_bytes()).await.unwrap();
            println!("Message Sent Successfully\n");
        }
        Err(e) => {
            println!("Connection failed: {}\n", e);
        }
    }
}