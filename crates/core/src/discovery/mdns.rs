use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo, TxtProperties};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{Duration, sleep};

use crate::logging::LogBus;
use crate::networking::client::connect_to_node;
use crate::peers::PeerRegistry;
use crate::tasks::task::Task;
use crate::tasks::tracker::TaskTracker;
use crate::utils::sysinfo::NodeSpecs;

/// Service type advertised on the local network. Public so tests / other
/// modules can reference the canonical value.
pub const SERVICE_TYPE: &str = "_sangam._udp.local.";

/// Extract the `node_id` TXT property from an mDNS announcement, if present.
pub fn extract_peer_node_id(properties: &TxtProperties) -> Option<String> {
    extract_txt_string(properties, "node_id")
}

/// Read a string-valued TXT property by key.
fn extract_txt_string(properties: &TxtProperties, key: &str) -> Option<String> {
    properties
        .iter()
        .find(|p| p.key() == key)
        .and_then(|p| p.val())
        .map(|v| String::from_utf8_lossy(v).to_string())
}

/// Read a numeric TXT property by key. Returns `None` if missing or
/// unparseable — peers running older builds may omit some properties.
fn extract_txt_number<T: std::str::FromStr>(properties: &TxtProperties, key: &str) -> Option<T> {
    extract_txt_string(properties, key).and_then(|s| s.parse().ok())
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

// `start_discovery` is the top-level orchestration entry for the
// discovery loop; every parameter here is independently needed and the
// function has exactly one caller (`run()` in lib.rs), so the value of
// bundling them into a context struct is low. Suppressing the lint is
// the honest choice.
#[allow(clippy::too_many_arguments)]
pub async fn start_discovery(
    node_id: String,
    local_ip: IpAddr,
    port: u16,
    shutdown: Arc<AtomicBool>,
    demo_task: Task,
    peers: Arc<PeerRegistry>,
    logs: Arc<LogBus>,
    tracker: Arc<TaskTracker>,
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

    // Channel carries (target_addr, our_node_id, task, peer_node_id).
    // peer_node_id is what the tracker stamps as the "worker" for the
    // outbound record, so the dashboard can show "task X ran on peer Y".
    let (tx, mut rx) = tokio::sync::mpsc::channel::<(String, String, Task, Option<String>)>(5);

    // Spawn a bounded worker that connects to discovered peers.
    let worker_tracker = tracker.clone();
    let worker_handle = tokio::spawn(async move {
        while let Some((target, our_node_id, task, peer_id)) = rx.recv().await {
            connect_to_node(target, our_node_id, task, peer_id, worker_tracker.clone()).await;
        }
    });

    while !shutdown.load(Ordering::Relaxed) {
        while let Ok(event) = receiver.try_recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    let peer_node_id = extract_peer_node_id(info.get_properties());

                    // Self-detection: skip our own broadcast no matter which
                    // network interface (WiFi, Ethernet, Docker bridge…) it
                    // arrives on. Comparing IPs alone is unreliable because
                    // a single host can advertise itself on many addresses.
                    if peer_node_id.as_deref() == Some(node_id.as_str()) {
                        continue;
                    }

                    // Pick the LAN address before doing anything else so
                    // both the connection attempt and the registry record
                    // share a single source of truth.
                    let picked = pick_peer_address(info.get_addresses().iter()).copied();

                    // Mirror every resolution into the peer registry so
                    // the UI sees the peer immediately, even if we've
                    // already kicked off a connection earlier this session.
                    let mut newly_seen = false;
                    if let (Some(pid), Some(addr)) = (&peer_node_id, picked) {
                        let cpu_threads = extract_txt_number::<u32>(info.get_properties(), "cpu");
                        let ram_gib = extract_txt_number::<u64>(info.get_properties(), "ram_gib");
                        // Track whether this is the first time we've seen
                        // this peer in this session so we only emit one
                        // "peer joined" log line per peer (re-resolutions
                        // happen frequently and would spam the console).
                        newly_seen = !connected_peers.contains(pid);
                        peers
                            .upsert(
                                pid.clone(),
                                info.get_fullname().to_string(),
                                SocketAddr::new(addr, info.get_port()),
                                cpu_threads,
                                ram_gib,
                            )
                            .await;
                    }

                    if newly_seen {
                        logs.info(
                            "discovery",
                            format!(
                                "Peer joined: {} ({})",
                                info.get_fullname(),
                                picked
                                    .map(|a| a.to_string())
                                    .unwrap_or_else(|| "unknown addr".to_string())
                            ),
                        )
                        .await;
                    }

                    // De-dup: only kick off the demo connection once per
                    // peer per session. The registry update above still
                    // happens on every resolution so freshness is correct.
                    if let Some(ref pid) = peer_node_id
                        && !connected_peers.insert(pid.clone())
                    {
                        continue;
                    }

                    println!("-----------------------------------------------");
                    println!("NODE DISCOVERED");
                    println!("-----------------------------------------------");
                    println!("Name : {}", info.get_fullname());

                    if let Some(addr) = picked {
                        println!("IP   : {}", addr);
                        let target = format!("{}:{}", addr, info.get_port());

                        // Mint a fresh task_id per send so the tracker can
                        // distinguish "demo task to peer A" from "demo
                        // task to peer B" — they share base behaviour but
                        // are independent lifecycle records.
                        let task = Task {
                            task_id: format!(
                                "{}-{}",
                                demo_task.task_id,
                                peer_node_id
                                    .as_deref()
                                    .map(|p| &p[..p.len().min(8)])
                                    .unwrap_or("anon")
                            ),
                            ..demo_task.clone()
                        };

                        // Send to bounded channel; if the channel is full this
                        // will back-pressure discovery rather than spawn
                        // unbounded tasks.
                        let msg = (target, node_id.clone(), task, peer_node_id.clone());
                        if tx.try_send(msg).is_err() {
                            let warning = "Connection queue full — skipping peer for now";
                            eprintln!("[discovery] {}", warning);
                            logs.warn("discovery", warning).await;
                        }
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
                ServiceEvent::ServiceRemoved(_, fullname) => {
                    // We key the registry by node_id (UUID), but mDNS
                    // removal events only carry the service fullname.
                    // Drop any record whose stored name matches.
                    //
                    // Cheap because the registry is small (mesh size).
                    let snapshot = peers.list().await;
                    if let Some(victim) = snapshot.iter().find(|p| p.name == fullname) {
                        peers.remove(&victim.id).await;
                        connected_peers.remove(&victim.id);
                        let msg = format!("Peer left: {}", fullname);
                        println!("[discovery] {}", msg);
                        logs.info("discovery", msg).await;
                    }
                }
                _ => {}
            }
        }

        sleep(Duration::from_secs(2)).await;
    }

    // Drop the sender so the worker exits its loop once all queued
    // connections finish.
    drop(tx);
    let _ = worker_handle.await;

    // Graceful cleanup: tell the daemon to send Goodbye packets so peers
    // remove us from their caches immediately instead of waiting for TTL.
    println!("[discovery] Unregistering from mDNS...");
    if let Err(e) = mdns.shutdown() {
        eprintln!("[discovery] mDNS shutdown failed: {:?}", e);
    }
}
