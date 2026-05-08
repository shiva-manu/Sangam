use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo, TxtProperties};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{Duration, sleep};

use crate::networking::client::connect_to_node;
use crate::utils::sysinfo::NodeSpecs;

/// Service type advertised on the local network. Public so tests / other
/// modules can reference the canonical value.
pub const SERVICE_TYPE: &str = "_sangam._udp.local.";

/// Extract the `node_id` TXT property from an mDNS announcement, if present.
pub fn extract_peer_node_id(properties: &TxtProperties) -> Option<String> {
    properties
        .iter()
        .find(|p| p.key() == "node_id")
        .and_then(|p| p.val())
        .map(|v| String::from_utf8_lossy(v).to_string())
}

/// Choose the most useful address from a peer's set of advertised addresses.
///
/// `mdns-sd` returns a `HashSet<IpAddr>` so iteration order is arbitrary.
/// Picking `iter().next()` blindly can land on a Docker bridge or loopback
/// when the peer is also reachable on the LAN. Preference order:
///
/// 1. IPv4 non-loopback, non-link-local (the typical LAN address)
/// 2. Any non-loopback (covers IPv6-only peers)
/// 3. Whatever's first (guarantees we still try *something*)
pub fn pick_peer_address<'a, I>(addresses: I) -> Option<&'a IpAddr>
where
    I: IntoIterator<Item = &'a IpAddr> + Clone,
{
    let is_link_local_v4 = |a: &IpAddr| match a {
        IpAddr::V4(v4) => v4.is_link_local(),
        IpAddr::V6(_) => false,
    };

    addresses
        .clone()
        .into_iter()
        .find(|a| a.is_ipv4() && !a.is_loopback() && !is_link_local_v4(a))
        .or_else(|| addresses.clone().into_iter().find(|a| !a.is_loopback()))
        .or_else(|| addresses.into_iter().next())
}

pub async fn start_discovery(
    node_id: String,
    local_ip: IpAddr,
    port: u16,
    shutdown: Arc<AtomicBool>,
) {
    let mdns = ServiceDaemon::new().expect("Failed to create mDNS daemon");

    let instance_name = format!("sangam-node-{}", &node_id[..8]);
    let host_name = format!("{}.local.", instance_name);

    // Detect actual hardware instead of advertising hard-coded specs so
    // peers can make sensible scheduling decisions.
    let specs = NodeSpecs::detect();

    let properties = HashMap::from([
        ("node_id".to_string(), node_id.clone()),
        ("cpu".to_string(), specs.cpu_threads.to_string()),
        ("ram_gib".to_string(), specs.ram_gib.to_string()),
    ]);

    let service_info = ServiceInfo::new(
        SERVICE_TYPE,
        &instance_name,
        &host_name,
        local_ip,
        port,
        properties,
    )
    .expect("Failed to build service info");

    mdns.register(service_info)
        .expect("Failed to register node");

    println!("Node Registered Successfully");
    println!("Node ID    : {}", node_id);
    println!("IP Address : {}", local_ip);
    println!("Port       : {}", port);
    println!(
        "Specs      : {} CPU thread(s), {} GiB RAM",
        specs.cpu_threads, specs.ram_gib
    );

    println!("\nSearching for nearby Sangam Nodes...\n");

    let receiver = mdns.browse(SERVICE_TYPE).expect("Failed to start browse");

    // Track peer node_ids we have already attempted to contact so we don't
    // hammer the same peer every time mDNS re-resolves it.
    let mut connected_peers: HashSet<String> = HashSet::new();

    while !shutdown.load(Ordering::Relaxed) {
        while let Ok(event) = receiver.try_recv() {
            if let ServiceEvent::ServiceResolved(info) = event {
                let peer_node_id = extract_peer_node_id(info.get_properties());

                // Self-detection: skip our own broadcast no matter which
                // network interface (WiFi, Ethernet, Docker bridge…) it
                // arrives on. Comparing IPs alone is unreliable because a
                // single host can advertise itself on many addresses.
                if peer_node_id.as_deref() == Some(node_id.as_str()) {
                    continue;
                }

                // De-dup: only act on each peer once per discovery session.
                if let Some(ref pid) = peer_node_id
                    && !connected_peers.insert(pid.clone())
                {
                    continue;
                }

                println!("-----------------------------------------------");
                println!("NODE DISCOVERED");
                println!("-----------------------------------------------");
                println!("Name : {}", info.get_fullname());

                // Pick the most useful address (LAN > any > anything) so
                // we don't end up dialing a Docker bridge by mistake.
                if let Some(addr) = pick_peer_address(info.get_addresses().iter()) {
                    println!("IP   : {}", addr);
                    let target = format!("{}:{}", addr, info.get_port());
                    tokio::spawn(connect_to_node(target, node_id.clone()));
                }

                println!("Port : {}", info.get_port());

                println!("Properties:");
                for prop in info.get_properties().iter() {
                    match prop.val() {
                        Some(v) => {
                            println!("  {} => {}", prop.key(), String::from_utf8_lossy(v))
                        }
                        None => println!("  {} => <empty>", prop.key()),
                    }
                }

                println!("------------------------------------------------\n");
            }
        }

        sleep(Duration::from_secs(2)).await;
    }

    // Graceful cleanup: tell the daemon to send Goodbye packets so peers
    // remove us from their caches immediately instead of waiting for TTL.
    println!("[discovery] Unregistering from mDNS...");
    if let Err(e) = mdns.shutdown() {
        eprintln!("[discovery] mDNS shutdown failed: {:?}", e);
    }
}
