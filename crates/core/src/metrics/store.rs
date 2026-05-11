//! Thread-safe ring buffer of `MetricsSample`s.
//!
//! Sized to hold one minute of samples at the default 1-second cadence
//! (60 entries). Push is O(1) amortized; reads clone out into a fresh
//! `Vec` so callers don't hold the lock while iterating.

use std::collections::VecDeque;

use tokio::sync::RwLock;

use super::sample::MetricsSample;

/// Default ring-buffer capacity. 60 samples × 1s cadence = 1 minute of
/// history, which matches the live charts in the dashboard.
pub const DEFAULT_HISTORY_CAPACITY: usize = 60;

/// Thread-safe FIFO of recent metrics samples.
pub struct MetricsStore {
    inner: RwLock<MetricsStoreInner>,
}

struct MetricsStoreInner {
    samples: VecDeque<MetricsSample>,
    capacity: usize,
}

impl MetricsStore {
    /// Create a store with `capacity` slots.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            inner: RwLock::new(MetricsStoreInner {
                samples: VecDeque::with_capacity(cap),
                capacity: cap,
            }),
        }
    }

    /// Append a sample, evicting the oldest one once we hit capacity.
    pub async fn push(&self, sample: MetricsSample) {
        let mut inner = self.inner.write().await;
        if inner.samples.len() == inner.capacity {
            inner.samples.pop_front();
        }
        inner.samples.push_back(sample);
    }

    /// The most recent sample, if any have been recorded yet.
    pub async fn latest(&self) -> Option<MetricsSample> {
        self.inner.read().await.samples.back().cloned()
    }

    /// Clone every retained sample, oldest first.
    pub async fn history(&self) -> Vec<MetricsSample> {
        self.inner.read().await.samples.iter().cloned().collect()
    }

    /// Number of retained samples (≤ capacity).
    pub async fn len(&self) -> usize {
        self.inner.read().await.samples.len()
    }

    /// True if no samples have been pushed yet.
    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.samples.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(t: u64, cpu: f32) -> MetricsSample {
        MetricsSample {
            timestamp_ms: t,
            cpu_pct: cpu,
            ram_used_mb: 0,
            ram_total_mb: 8192,
            net_rx_kbps: 0.0,
            net_tx_kbps: 0.0,
        }
    }

    #[tokio::test]
    async fn empty_store_returns_no_history() {
        let store = MetricsStore::new(10);
        assert!(store.is_empty().await);
        assert_eq!(store.len().await, 0);
        assert!(store.latest().await.is_none());
        assert!(store.history().await.is_empty());
    }

    #[tokio::test]
    async fn push_and_latest_return_most_recent_sample() {
        let store = MetricsStore::new(10);
        store.push(s(1, 10.0)).await;
        store.push(s(2, 20.0)).await;
        assert_eq!(store.latest().await.unwrap().cpu_pct, 20.0);
        assert_eq!(store.len().await, 2);
    }

    #[tokio::test]
    async fn history_returns_samples_in_oldest_to_newest_order() {
        let store = MetricsStore::new(10);
        for i in 0..3 {
            store.push(s(i, i as f32)).await;
        }
        let h = store.history().await;
        assert_eq!(h.len(), 3);
        assert_eq!(h[0].timestamp_ms, 0);
        assert_eq!(h[2].timestamp_ms, 2);
    }

    #[tokio::test]
    async fn ring_buffer_evicts_oldest_when_at_capacity() {
        let store = MetricsStore::new(3);
        for i in 0..5 {
            store.push(s(i, 0.0)).await;
        }
        let h = store.history().await;
        // After 5 pushes into a 3-slot ring, only timestamps 2,3,4 remain.
        assert_eq!(h.len(), 3);
        assert_eq!(h[0].timestamp_ms, 2);
        assert_eq!(h[1].timestamp_ms, 3);
        assert_eq!(h[2].timestamp_ms, 4);
    }

    #[tokio::test]
    async fn capacity_zero_is_clamped_to_one_to_avoid_pathological_state() {
        let store = MetricsStore::new(0);
        store.push(s(0, 0.0)).await;
        store.push(s(1, 0.0)).await;
        assert_eq!(store.len().await, 1);
        assert_eq!(store.latest().await.unwrap().timestamp_ms, 1);
    }
}
