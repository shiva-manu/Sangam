//! TCP networking layer for node-to-node communication.
//!
//! This module provides the low-level transport used by Sangam nodes to send
//! and receive [`crate::models::message::NodeMessage`] values over the
//! network. All messages are framed as newline-delimited JSON so they can be
//! streamed over a plain TCP connection without a custom binary protocol.
//!
//! # Sub-modules
//! - [`client`] — outbound TCP connection logic; used to dial a remote peer
//!   and deliver a single message.
//! - [`server`] — inbound TCP listener logic; accepts connections from peers
//!   and dispatches incoming messages to the rest of the runtime.

pub mod client;
pub mod server;
