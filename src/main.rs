mod discovery;
mod networking;
mod models;
mod utils;

use discovery::mdns::start_discovery;
use networking::server::start_tcp_server;
use utils::banner::show_banner;

use uuid::Uuid;

#[tokio::main]
async fn main() {
    show_banner();

    let node_id = Uuid::new_v4().to_string();
    let local_ip = local_ip_address::local_ip().unwrap();
    let port: u16 = 8080;

    // Start TCP server in background
    tokio::spawn(start_tcp_server(port));

    // Start mDNS discovery (blocks forever)
    start_discovery(node_id, local_ip, port).await;
}