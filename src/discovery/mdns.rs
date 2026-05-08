use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

use crate::networking::client::connect_to_node;

pub async fn start_discovery(node_id: String, local_ip: std::net::IpAddr, port: u16) {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");

    let service_type = "_sangam._udp.local.";
    let instance_name = format!("sangam-node-{}", &node_id[..8]);
    let host_name = format!("{}.local.", instance_name);

    let properties = HashMap::from([
        ("node_id".to_string(), node_id.clone()),
        ("cpu".to_string(), "8".to_string()),
        ("ram".to_string(), "16GB".to_string()),
    ]);

    let service_info = ServiceInfo::new(
        service_type,
        &instance_name,
        &host_name,
        local_ip,
        port,
        properties,
    )
    .unwrap();

    mdns.register(service_info)
        .expect("Failed to register node");

    println!("Node Registered Successfully");
    println!("Node ID    : {}", node_id);
    println!("IP Address : {}", local_ip);
    println!("Port       : {}", port);

    println!("\nSearching for nearby Sangam Nodes...\n");

    let receiver = mdns.browse(service_type).unwrap();

    loop {
        while let Ok(event) = receiver.try_recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("-----------------------------------------------");
                    println!("NODE DISCOVERED");
                    println!("-----------------------------------------------");
                    println!("Name : {}", info.get_fullname());

                    for addr in info.get_addresses() {
                        println!("IP   : {}", addr);

                        if addr.to_string() == local_ip.to_string() {
                            continue;
                        }

                        let target = format!("{}:{}", addr, info.get_port());
                        tokio::spawn(connect_to_node(target, node_id.clone()));
                    }

                    println!("Port : {}", info.get_port());

                    println!("Properties:");
                    for prop in info.get_properties().iter() {
                        println!("  {} => {:?}", prop.key(), prop.val());
                    }

                    println!("------------------------------------------------\n");
                }
                _ => {}
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}