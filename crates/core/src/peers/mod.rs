//! Peer registry — stateful tracking of discovered Sangam nodes.
//!
//! mDNS gives us *transient* events: ServiceResolved when a peer announces
//! itself, ServiceRemoved when its TTL lapses or it sends a Goodbye. The
//! registry turns that event stream into a queryable snapshot the UI can
//! render: who is here right now, when did we last hear from them, and is
//! their liveness Online / Stale / Offline.

pub mod registry;

pub use registry::{OFFLINE_AFTER, Peer, PeerRegistry, PeerStatus, STALE_AFTER};
