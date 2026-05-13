//! Peer discovery via mDNS (Multicast DNS).
//!
//! This module is responsible for advertising the local Sangam node on the
//! local network and for detecting other Sangam nodes that are doing the
//! same. It uses mDNS so that no central registry or static configuration
//! is required — nodes find each other automatically as long as they share
//! the same multicast-capable network segment.
//!
//! # Sub-modules
//! - [`mdns`] — core mDNS advertisement and listener logic.

pub mod mdns;
