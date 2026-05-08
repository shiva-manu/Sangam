use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn start_tcp_server(port: u16) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to start TCP server");
    println!("TCP Server Running on port {}\n", port);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        println!("Incoming connection from {}\n", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let received = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received Message:");
                    println!("{}", received);
                    println!();
                }
                _ => {}
            }
        });
    }
}
