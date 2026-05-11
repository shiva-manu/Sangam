//! Background task that samples local resource usage on a fixed cadence.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use sysinfo::{Networks, System};

use super::sample::MetricsSample;
use super::store::MetricsStore;

/// Default sampling interval. 1 second matches typical infra dashboards
/// (Datadog, Vercel) and keeps the chart smooth without flooding tokio
/// with timer events.
pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(1);

/// Sample local CPU / RAM / network throughput into `store` until
/// `shutdown` flips to `true`.
///
/// The first iteration sleeps before sampling so that `sysinfo`'s
/// network counters (which return *bytes since last refresh*) yield
/// meaningful deltas instead of cumulative-since-boot values.
///
/// Spawn as a Tokio task; this future never returns until shutdown.
pub async fn run_collector(
    store: Arc<MetricsStore>,
    shutdown: Arc<AtomicBool>,
    interval: Duration,
) {
    let interval = if interval.is_zero() {
        DEFAULT_INTERVAL
    } else {
        interval
    };

    // `System::new()` is empty — much faster than `new_all()` which would
    // also scan every running process (which we don't need). We then prime
    // just the metrics we care about so the first refresh in the loop
    // produces a meaningful delta.
    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.refresh_memory();
    let mut nets = Networks::new_with_refreshed_list();

    // Establish a baseline interval before the first sample so network
    // counters return "bytes during this interval" instead of cumulative.
    tokio::time::sleep(interval).await;

    while !shutdown.load(Ordering::Relaxed) {
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        nets.refresh();

        let cpu_pct = sys.global_cpu_usage();
        let ram_total_mb = bytes_to_mib(sys.total_memory());
        let ram_used_mb = bytes_to_mib(sys.used_memory());

        let (rx_bytes, tx_bytes) = nets
            .values()
            .map(|n| (n.received(), n.transmitted()))
            .fold((0u64, 0u64), |(a, b), (r, t)| {
                (a.saturating_add(r), b.saturating_add(t))
            });

        // Convert "bytes since last refresh" into KiB/s. Guard against
        // a zero interval just in case a caller passes an unreasonable
        // duration (we already clamped, but defence-in-depth is cheap).
        let interval_secs = interval.as_secs_f64().max(1e-3);
        let net_rx_kbps = (rx_bytes as f64 / 1024.0) / interval_secs;
        let net_tx_kbps = (tx_bytes as f64 / 1024.0) / interval_secs;

        store
            .push(MetricsSample::now(
                cpu_pct,
                ram_used_mb,
                ram_total_mb,
                net_rx_kbps,
                net_tx_kbps,
            ))
            .await;

        tokio::time::sleep(interval).await;
    }
}

/// Convert a byte count to mebibytes, rounding down. Pulled out of the
/// hot loop to keep the cast explicit and unit-testable.
fn bytes_to_mib(bytes: u64) -> u64 {
    bytes / (1024 * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_mib_truncates_consistently() {
        assert_eq!(bytes_to_mib(0), 0);
        assert_eq!(bytes_to_mib(1024 * 1024 - 1), 0);
        assert_eq!(bytes_to_mib(1024 * 1024), 1);
        assert_eq!(bytes_to_mib(8 * 1024 * 1024 * 1024), 8 * 1024);
    }

    /// Smoke test: spinning up the collector for a few intervals produces
    /// *some* samples without panicking. We intentionally don't assert
    /// on values — those depend on real OS state — only on the wiring
    /// (init, refresh cadence, store push, shutdown handling).
    ///
    /// Intervals and waits are generous to keep CI runners (which may
    /// have slow `sysinfo` initialization on cold disks) reliable.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn collector_pushes_samples_until_shutdown() {
        let store = Arc::new(MetricsStore::new(8));
        let shutdown = Arc::new(AtomicBool::new(false));
        let interval = Duration::from_millis(100);

        let store_clone = store.clone();
        let shutdown_clone = shutdown.clone();
        let handle = tokio::spawn(async move {
            run_collector(store_clone, shutdown_clone, interval).await;
        });

        // Wait long enough for: init + baseline interval + several pushes.
        tokio::time::sleep(Duration::from_millis(800)).await;
        shutdown.store(true, Ordering::Relaxed);
        tokio::time::sleep(interval * 2).await;
        handle.abort();
        let _ = handle.await;

        let len = store.len().await;
        assert!(
            len >= 1,
            "expected at least one sample after 800ms of collection, got {}",
            len
        );
    }
}
