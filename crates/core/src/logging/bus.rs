//! `LogBus` — combined ring buffer + broadcast channel for runtime events.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tokio::sync::{RwLock, broadcast};

/// Default ring-buffer capacity. 200 entries is enough to populate the
/// console on app launch without using meaningful memory; live events
/// stream in over the broadcast channel after that.
pub const DEFAULT_HISTORY_CAPACITY: usize = 200;

/// Default broadcast channel depth. Subscribers that fall this far
/// behind will receive `Lagged` errors; the bridge re-syncs by pulling
/// `recent()` rather than panicking.
const DEFAULT_BROADCAST_DEPTH: usize = 256;

/// Severity levels — small fixed set so the UI can colour-code easily.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// One structured log entry.
#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    /// Wall-clock milliseconds since the Unix epoch.
    pub timestamp_ms: u64,
    pub level: LogLevel,
    /// Module that produced the entry: "discovery" | "networking" |
    /// "tasks" | "runtime". Free-form so future modules don't need
    /// changes here, but keep it short.
    pub source: String,
    pub message: String,
}

impl LogEntry {
    /// Build an entry stamped with the current wall-clock time.
    pub fn now(level: LogLevel, source: impl Into<String>, message: impl Into<String>) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            timestamp_ms,
            level,
            source: source.into(),
            message: message.into(),
        }
    }
}

/// Shared logging sink. Wrap in `Arc` and pass into anything that emits.
///
/// Cheaply clonable subscribers can be obtained via `subscribe()`.
pub struct LogBus {
    sender: broadcast::Sender<LogEntry>,
    history: RwLock<HistoryInner>,
}

struct HistoryInner {
    entries: VecDeque<LogEntry>,
    capacity: usize,
}

impl LogBus {
    /// Build a bus with the supplied history capacity.
    pub fn new(history_capacity: usize) -> Self {
        let cap = history_capacity.max(1);
        let (sender, _) = broadcast::channel(DEFAULT_BROADCAST_DEPTH);
        Self {
            sender,
            history: RwLock::new(HistoryInner {
                entries: VecDeque::with_capacity(cap),
                capacity: cap,
            }),
        }
    }

    /// Build a bus with the default capacity. Useful for tests / sane
    /// production defaults.
    pub fn with_defaults() -> Self {
        Self::new(DEFAULT_HISTORY_CAPACITY)
    }

    /// Subscribe to the live event stream. Subscribers that fall more
    /// than the broadcast depth behind get `RecvError::Lagged` — the
    /// caller should then re-sync via `recent()`.
    pub fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.sender.subscribe()
    }

    /// Emit an entry. Cheap, never blocks, never errors visibly:
    ///   * Always appends to the ring buffer.
    ///   * Broadcast-send is best-effort; failure (no subscribers,
    ///     channel closed) is silent because the producer has nothing
    ///     useful to do about it.
    pub async fn emit(&self, entry: LogEntry) {
        {
            let mut h = self.history.write().await;
            if h.entries.len() == h.capacity {
                h.entries.pop_front();
            }
            h.entries.push_back(entry.clone());
        }
        let _ = self.sender.send(entry);
    }

    /// Snapshot of every retained entry, oldest first.
    pub async fn recent(&self) -> Vec<LogEntry> {
        self.history.read().await.entries.iter().cloned().collect()
    }

    /// Number of subscribers currently listening on the broadcast channel.
    /// Mostly useful for tests; producers should not gate emits on this.
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }

    // ---- Convenience emitters --------------------------------------

    /// Emit at `Info` level.
    pub async fn info(&self, source: impl Into<String>, message: impl Into<String>) {
        self.emit(LogEntry::now(LogLevel::Info, source, message))
            .await;
    }

    /// Emit at `Warn` level.
    pub async fn warn(&self, source: impl Into<String>, message: impl Into<String>) {
        self.emit(LogEntry::now(LogLevel::Warn, source, message))
            .await;
    }

    /// Emit at `Error` level.
    pub async fn error(&self, source: impl Into<String>, message: impl Into<String>) {
        self.emit(LogEntry::now(LogLevel::Error, source, message))
            .await;
    }

    /// Emit at `Debug` level.
    pub async fn debug(&self, source: impl Into<String>, message: impl Into<String>) {
        self.emit(LogEntry::now(LogLevel::Debug, source, message))
            .await;
    }
}

impl Default for LogBus {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn emit_appends_to_history() {
        let bus = LogBus::with_defaults();
        bus.info("discovery", "peer joined").await;
        let entries = bus.recent().await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source, "discovery");
        assert_eq!(entries[0].message, "peer joined");
        assert_eq!(entries[0].level, LogLevel::Info);
    }

    #[tokio::test]
    async fn convenience_methods_set_correct_levels() {
        let bus = LogBus::with_defaults();
        bus.debug("x", "d").await;
        bus.info("x", "i").await;
        bus.warn("x", "w").await;
        bus.error("x", "e").await;
        let entries = bus.recent().await;
        assert_eq!(
            entries.iter().map(|e| e.level).collect::<Vec<_>>(),
            vec![
                LogLevel::Debug,
                LogLevel::Info,
                LogLevel::Warn,
                LogLevel::Error
            ]
        );
    }

    #[tokio::test]
    async fn history_evicts_oldest_at_capacity() {
        let bus = LogBus::new(3);
        for i in 0..5 {
            bus.info("x", format!("msg-{}", i)).await;
        }
        let entries = bus.recent().await;
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].message, "msg-2");
        assert_eq!(entries[2].message, "msg-4");
    }

    #[tokio::test]
    async fn capacity_zero_is_clamped_to_one() {
        let bus = LogBus::new(0);
        bus.info("x", "first").await;
        bus.info("x", "second").await;
        let entries = bus.recent().await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message, "second");
    }

    #[tokio::test]
    async fn emit_with_no_subscribers_does_not_panic() {
        let bus = LogBus::with_defaults();
        // No subscribe() — broadcast send will fail silently.
        bus.info("x", "noop").await;
        assert_eq!(bus.recent().await.len(), 1);
    }

    #[tokio::test]
    async fn subscriber_receives_emitted_entries() {
        let bus = LogBus::with_defaults();
        let mut rx = bus.subscribe();
        bus.info("discovery", "hello").await;
        let received = rx.recv().await.unwrap();
        assert_eq!(received.source, "discovery");
        assert_eq!(received.message, "hello");
    }

    #[tokio::test]
    async fn multiple_subscribers_each_get_a_copy() {
        let bus = LogBus::with_defaults();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        bus.info("x", "broadcast").await;
        assert_eq!(rx1.recv().await.unwrap().message, "broadcast");
        assert_eq!(rx2.recv().await.unwrap().message, "broadcast");
    }

    #[tokio::test]
    async fn entry_serializes_to_expected_json_shape() {
        let entry = LogEntry {
            timestamp_ms: 1_700_000_000_000,
            level: LogLevel::Warn,
            source: "discovery".into(),
            message: "stale peer".into(),
        };
        let v = serde_json::to_value(&entry).unwrap();
        assert_eq!(v["timestamp_ms"], 1_700_000_000_000u64);
        assert_eq!(v["level"], "warn");
        assert_eq!(v["source"], "discovery");
        assert_eq!(v["message"], "stale peer");
    }

    #[test]
    fn level_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&LogLevel::Info).unwrap(), "\"info\"");
        assert_eq!(serde_json::to_string(&LogLevel::Warn).unwrap(), "\"warn\"");
        assert_eq!(
            serde_json::to_string(&LogLevel::Error).unwrap(),
            "\"error\""
        );
    }
}
