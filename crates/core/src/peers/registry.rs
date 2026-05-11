//! In-memory peer registry.
//!
//! Concurrency model: `Arc<PeerRegistry>` is shared between the discovery
//! task (which writes) and Tauri command handlers (which read). All access
//! goes through a single `RwLock<HashMap>`. Reads dominate writes, so the
//! `RwLock` (vs `Mutex`) is the right primitive.
//!
//! Liveness: we never delete a peer immediately. Instead, status decays
//! Online → Stale → Offline based on `last_seen`. The UI can show offline
//! peers (greyed out) so the user notices when a teammate's laptop goes
//! to sleep instead of having the row silently disappear. Truly stale
//! records are pruned by `evict_offline()` which the runtime calls
//! periodically.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::RwLock;

/// How long a peer can go unseen before we mark it `Stale`.
pub const STALE_AFTER: Duration = Duration::from_secs(30);

/// How long a peer can go unseen before we mark it `Offline`.
pub const OFFLINE_AFTER: Duration = Duration::from_secs(120);

/// How long a peer can stay `Offline` before `evict_offline` drops it.
const EVICTION_THRESHOLD: Duration = Duration::from_secs(600);

/// Liveness state, derived from `last_seen` at observation time.
///
/// We compute this on read rather than store it, so a peer transitions
/// to Stale automatically without needing a background ticker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PeerStatus {
    /// Seen recently — actively part of the mesh.
    Online,
    /// Not seen for a while but probably still around.
    Stale,
    /// Likely gone (sleeping laptop, network blip, app closed).
    Offline,
}

/// Pure function: classify an `elapsed` duration into a liveness state.
///
/// Lifted out of the registry so unit tests don't need to fake `Instant`.
pub fn classify(elapsed: Duration) -> PeerStatus {
    if elapsed < STALE_AFTER {
        PeerStatus::Online
    } else if elapsed < OFFLINE_AFTER {
        PeerStatus::Stale
    } else {
        PeerStatus::Offline
    }
}

/// Public, JSON-serializable view of a peer.
///
/// Intentionally separate from the internal `PeerRecord` (below) so we
/// can store non-`Serialize` types like `Instant` without leaking them
/// across the FFI boundary into Tauri / the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct Peer {
    pub id: String,
    pub name: String,
    pub addr: SocketAddr,
    /// CPU thread count advertised in the mDNS TXT record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_threads: Option<u32>,
    /// RAM in GiB advertised in the mDNS TXT record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_gib: Option<u64>,
    /// Seconds since we last received a packet from this peer.
    pub last_seen_secs_ago: u64,
    pub status: PeerStatus,
}

/// Internal record. Holds the raw `Instant` we use to compute status.
#[derive(Debug, Clone)]
struct PeerRecord {
    name: String,
    addr: SocketAddr,
    cpu_threads: Option<u32>,
    ram_gib: Option<u64>,
    last_seen: Instant,
}

/// Thread-safe map from `node_id` → peer state.
#[derive(Default)]
pub struct PeerRegistry {
    peers: RwLock<HashMap<String, PeerRecord>>,
}

impl PeerRegistry {
    /// Create an empty registry. Wrap in `Arc` for sharing across tasks.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or refresh a peer. Idempotent — calling it on every mDNS
    /// resolution event is the intended use.
    pub async fn upsert(
        &self,
        node_id: impl Into<String>,
        name: impl Into<String>,
        addr: SocketAddr,
        cpu_threads: Option<u32>,
        ram_gib: Option<u64>,
    ) {
        let node_id = node_id.into();
        let name = name.into();
        let mut peers = self.peers.write().await;
        peers
            .entry(node_id)
            .and_modify(|r| {
                r.name = name.clone();
                r.addr = addr;
                if cpu_threads.is_some() {
                    r.cpu_threads = cpu_threads;
                }
                if ram_gib.is_some() {
                    r.ram_gib = ram_gib;
                }
                r.last_seen = Instant::now();
            })
            .or_insert(PeerRecord {
                name,
                addr,
                cpu_threads,
                ram_gib,
                last_seen: Instant::now(),
            });
    }

    /// Remove a peer immediately (e.g. on `ServiceRemoved`).
    pub async fn remove(&self, node_id: &str) {
        self.peers.write().await.remove(node_id);
    }

    /// Snapshot of every known peer with status computed against `now`.
    pub async fn list(&self) -> Vec<Peer> {
        self.list_at(Instant::now()).await
    }

    /// Snapshot variant that takes an explicit `now`. Used by tests.
    pub async fn list_at(&self, now: Instant) -> Vec<Peer> {
        let peers = self.peers.read().await;
        let mut out: Vec<Peer> = peers
            .iter()
            .map(|(id, r)| {
                let elapsed = now.saturating_duration_since(r.last_seen);
                Peer {
                    id: id.clone(),
                    name: r.name.clone(),
                    addr: r.addr,
                    cpu_threads: r.cpu_threads,
                    ram_gib: r.ram_gib,
                    last_seen_secs_ago: elapsed.as_secs(),
                    status: classify(elapsed),
                }
            })
            .collect();
        // Stable order: online first, then by name. Keeps the UI from
        // shuffling rows on every poll.
        out.sort_by(|a, b| a.status.cmp(&b.status).then_with(|| a.name.cmp(&b.name)));
        out
    }

    /// Number of peers tracked (any status).
    pub async fn len(&self) -> usize {
        self.peers.read().await.len()
    }

    /// True when the registry has no peers.
    pub async fn is_empty(&self) -> bool {
        self.peers.read().await.is_empty()
    }

    /// Drop peers that have been Offline for longer than the eviction
    /// threshold. Call periodically from a background task — typical
    /// cadence is once a minute.
    pub async fn evict_offline(&self) {
        let now = Instant::now();
        self.peers
            .write()
            .await
            .retain(|_, r| now.saturating_duration_since(r.last_seen) < EVICTION_THRESHOLD);
    }
}

// `PeerStatus` order: Online < Stale < Offline. Used to sort peer lists.
impl Ord for PeerStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn rank(s: &PeerStatus) -> u8 {
            match s {
                PeerStatus::Online => 0,
                PeerStatus::Stale => 1,
                PeerStatus::Offline => 2,
            }
        }
        rank(self).cmp(&rank(other))
    }
}

impl PartialOrd for PeerStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn fixture_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5)), 8080)
    }

    #[test]
    fn classify_buckets_durations_into_status() {
        assert_eq!(classify(Duration::from_secs(0)), PeerStatus::Online);
        assert_eq!(
            classify(STALE_AFTER - Duration::from_millis(1)),
            PeerStatus::Online
        );
        assert_eq!(classify(STALE_AFTER), PeerStatus::Stale);
        assert_eq!(
            classify(OFFLINE_AFTER - Duration::from_millis(1)),
            PeerStatus::Stale
        );
        assert_eq!(classify(OFFLINE_AFTER), PeerStatus::Offline);
        assert_eq!(classify(Duration::from_secs(3600)), PeerStatus::Offline);
    }

    #[test]
    fn peer_status_orders_online_before_stale_before_offline() {
        let mut v = vec![PeerStatus::Offline, PeerStatus::Online, PeerStatus::Stale];
        v.sort();
        assert_eq!(
            v,
            vec![PeerStatus::Online, PeerStatus::Stale, PeerStatus::Offline]
        );
    }

    #[tokio::test]
    async fn upsert_inserts_a_new_peer() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), Some(8), Some(16))
            .await;
        let peers = reg.list().await;
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].id, "nid-1");
        assert_eq!(peers[0].name, "alex-mbp");
        assert_eq!(peers[0].cpu_threads, Some(8));
        assert_eq!(peers[0].ram_gib, Some(16));
        assert_eq!(peers[0].status, PeerStatus::Online);
    }

    #[tokio::test]
    async fn upsert_refreshes_last_seen_for_existing_peer() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        // Force the recorded last_seen into the past.
        {
            let mut peers = reg.peers.write().await;
            let rec = peers.get_mut("nid-1").unwrap();
            rec.last_seen = Instant::now() - Duration::from_secs(60);
        }
        // Re-upsert; this should bring it back to Online.
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        assert_eq!(reg.list().await[0].status, PeerStatus::Online);
    }

    #[tokio::test]
    async fn upsert_preserves_existing_specs_when_called_with_none() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), Some(8), Some(16))
            .await;
        // Second announcement omits specs (e.g. a trimmed TXT record).
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        let peers = reg.list().await;
        assert_eq!(peers[0].cpu_threads, Some(8));
        assert_eq!(peers[0].ram_gib, Some(16));
    }

    #[tokio::test]
    async fn remove_drops_the_peer() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        assert_eq!(reg.len().await, 1);
        reg.remove("nid-1").await;
        assert!(reg.is_empty().await);
    }

    #[tokio::test]
    async fn list_at_reports_stale_after_threshold() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        let now = Instant::now() + STALE_AFTER + Duration::from_secs(1);
        let peers = reg.list_at(now).await;
        assert_eq!(peers[0].status, PeerStatus::Stale);
    }

    #[tokio::test]
    async fn list_at_reports_offline_after_threshold() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), None, None)
            .await;
        let now = Instant::now() + OFFLINE_AFTER + Duration::from_secs(1);
        let peers = reg.list_at(now).await;
        assert_eq!(peers[0].status, PeerStatus::Offline);
        // Last-seen is rendered as a u64 of seconds, so we never lie
        // about freshness even when the peer is long gone.
        assert!(peers[0].last_seen_secs_ago >= OFFLINE_AFTER.as_secs());
    }

    #[tokio::test]
    async fn list_sorts_online_peers_before_offline() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-online", "zeta", fixture_addr(), None, None)
            .await;
        reg.upsert("nid-offline", "alpha", fixture_addr(), None, None)
            .await;
        // Backdate the second peer.
        {
            let mut peers = reg.peers.write().await;
            peers.get_mut("nid-offline").unwrap().last_seen =
                Instant::now() - OFFLINE_AFTER - Duration::from_secs(1);
        }
        let peers = reg.list().await;
        assert_eq!(peers[0].name, "zeta", "online peer comes first");
        assert_eq!(peers[1].name, "alpha");
    }

    #[tokio::test]
    async fn evict_offline_drops_only_long_gone_peers() {
        let reg = PeerRegistry::new();
        reg.upsert("recent", "a", fixture_addr(), None, None).await;
        reg.upsert("ancient", "b", fixture_addr(), None, None).await;
        {
            let mut peers = reg.peers.write().await;
            peers.get_mut("ancient").unwrap().last_seen =
                Instant::now() - EVICTION_THRESHOLD - Duration::from_secs(1);
        }
        reg.evict_offline().await;
        let ids: Vec<String> = reg.list().await.into_iter().map(|p| p.id).collect();
        assert_eq!(ids, vec!["recent"]);
    }

    #[tokio::test]
    async fn peer_serializes_to_expected_json_shape() {
        let reg = PeerRegistry::new();
        reg.upsert("nid-1", "alex-mbp", fixture_addr(), Some(8), Some(16))
            .await;
        let peer = &reg.list().await[0];
        let json = serde_json::to_value(peer).unwrap();
        // Field names match the TS NodeInfo-style contract the UI expects.
        assert_eq!(json["id"], "nid-1");
        assert_eq!(json["name"], "alex-mbp");
        assert_eq!(json["status"], "online");
        assert_eq!(json["cpu_threads"], 8);
        assert_eq!(json["ram_gib"], 16);
        assert!(json["last_seen_secs_ago"].is_u64());
        assert_eq!(json["addr"], "192.168.1.5:8080");
    }
}
