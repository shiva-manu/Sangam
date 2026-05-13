//! Wire-format message types exchanged between Sangam nodes over TCP.
//!
//! Every value that travels across a TCP connection between two nodes is
//! encoded as a [`message::NodeMessage`] and serialized to newline-delimited
//! JSON. This module collects all such types so the rest of the codebase has
//! a single, authoritative place to look for the network protocol.
//!
//! # Sub-modules
//! - [`message`] — the [`message::NodeMessage`] envelope and its
//!   [`message::MessageType`] payload variants.

pub mod message;
