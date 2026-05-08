use mdns_sd::{ServiceDaemon,ServiceEvent,ServiceInfo};
use std::collections::HashMap;
use tokio::time::{sleep,Duration};
use uuid::Uuid;

#[tokio::main]
async fn main(){
    println!("\n=======================================================");
    println!("                     Sangam v0.0.1                       ");
    println!("=======================================================\n");

    // Create mDNS daemon
    let mdns=ServiceDaemon::new().expect("Failed to create mDNS daemon");

    // Generate unique node ID
    let node_id=Uuid::new_v4().to_string();
    
    // Service type
    let service_type="_sangam._udp.local.";

    // Node instance name
    let instance_name=format!("sangam-node-{}",&node_id[..8]);

    // Local host name
    let host_name="sangam.local.";

    // Get local IP address
    let local_ip=local_ip_address::local_ip().unwrap();

    // Node listening port
    let port=8080;

    // Node metadata
    let properties=HashMap::from([
        ("node_id".to_string(),node_id.clone()),
        ("cpu".to_string(),"8".to_string()),
        ("ram".to_string(),"16GB".to_string()),
    ]);

    // Create service info 
    let service_info=ServiceInfo::new(
        service_type,
        &instance_name,
        host_name,
        local_ip,
        port,
        properties,
    )
    .unwrap();

    // Register node on network
    mdns.register(service_info)
        .expect("Failed to register node");
    
    println!("Node Registered Successfully");
    println!("Node ID    : {}",node_id);
    println!("IP Address : {}",local_ip);
    println!("Port       : {}",port);

    println!("\nSearching for nearby Sangam Nodes...\n");

    // Browse network for nearby nodes
    let recevier=mdns.browse(service_type).unwrap();

    loop{
        while let Ok(event)=recevier.try_recv(){
            match event{
                ServiceEvent::ServiceResolved(info)=>{
                    println!("-----------------------------------------------");
                    println!("NODE DISCOVERED");
                    println!("-----------------------------------------------");
                    println!("Name : {}",info.get_fullname());

                    for addr in info.get_addresses(){
                    println!("IP  : {}",addr);
                    }
                    println!("Port : {}",info.get_port());

                    println!("Properties:");

                    for prop in info.get_properties().iter(){
                        println!(" {} => {:?}",prop.key(),prop.val());
                    }
                    println!("------------------------------------------------\n");
                }
                _=>{}
            }
        }
        sleep(Duration::from_secs(2)).await;
    }
}