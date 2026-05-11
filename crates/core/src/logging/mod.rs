//! Structured logging bus for runtime events.
//!
//! The dashboard's "Distributed Runtime Console" needs a stream of
//! events richer than plain `println!`: each entry should carry a
//! timestamp, level, source module, and message so the UI can colour,
//! filter, and group them.
//!
//! `LogBus` is the single sink. Producers (discovery, networking, task
//! executor) call `bus.info("source", "msg")` etc. Consumers can either
//! subscribe to the live `tokio::sync::broadcast` channel for tail-style
//! rendering or pull `recent()` for an initial backfill on console open.
//!
//! Two delivery modes share the same writes so a late subscriber never
//! sees a hole: every emit appends to the ring buffer *before* it
//! publishes to the broadcast channel.

pub mod bus;

pub use bus::{DEFAULT_HISTORY_CAPACITY, LogBus, LogEntry, LogLevel};
