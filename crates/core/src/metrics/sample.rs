//! Single point-in-time metrics snapshot.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

/// One sample of local-machine resource usage.
///
/// Field naming follows the same `snake_case` convention the rest of
/// the Tauri command surface uses, so the UI can deserialize with
/// `serde_json` and render directly into charts.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MetricsSample {
    /// Unix epoch milliseconds. Suitable for charting on a time axis.
    pub timestamp_ms: u64,
    /// Global CPU usage 0.0..=100.0 (sysinfo's `global_cpu_usage`).
    pub cpu_pct: f32,
    /// RAM in use (MiB).
    pub ram_used_mb: u64,
    /// Total RAM (MiB). Stored on every sample so the UI doesn't need
    /// a second command for the static figure — it's tiny and avoids
    /// a "what if total_memory drifts?" edge case on hot-plug systems.
    pub ram_total_mb: u64,
    /// Network receive throughput in KiB/s (sum across interfaces).
    pub net_rx_kbps: f64,
    /// Network transmit throughput in KiB/s (sum across interfaces).
    pub net_tx_kbps: f64,
}

impl MetricsSample {
    /// Helper to stamp a sample with the current wall clock.
    ///
    /// Pulled into a function so tests that need a deterministic
    /// timestamp can construct samples directly without going through
    /// `SystemTime`.
    pub fn now(
        cpu_pct: f32,
        ram_used_mb: u64,
        ram_total_mb: u64,
        net_rx_kbps: f64,
        net_tx_kbps: f64,
    ) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            timestamp_ms,
            cpu_pct,
            ram_used_mb,
            ram_total_mb,
            net_rx_kbps,
            net_tx_kbps,
        }
    }

    /// RAM usage as a percentage. Convenience for the UI.
    pub fn ram_pct(&self) -> f32 {
        if self.ram_total_mb == 0 {
            0.0
        } else {
            (self.ram_used_mb as f32 / self.ram_total_mb as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_pct_handles_zero_total_without_panicking() {
        let s = MetricsSample::now(0.0, 100, 0, 0.0, 0.0);
        assert_eq!(s.ram_pct(), 0.0);
    }

    #[test]
    fn ram_pct_computes_correctly_for_normal_inputs() {
        let s = MetricsSample::now(0.0, 4096, 8192, 0.0, 0.0);
        assert!((s.ram_pct() - 50.0).abs() < 0.01);
    }

    #[test]
    fn sample_serializes_to_expected_json_shape() {
        let s = MetricsSample {
            timestamp_ms: 1_700_000_000_000,
            cpu_pct: 42.5,
            ram_used_mb: 4096,
            ram_total_mb: 8192,
            net_rx_kbps: 12.5,
            net_tx_kbps: 0.0,
        };
        let v = serde_json::to_value(&s).unwrap();
        assert_eq!(v["timestamp_ms"], 1_700_000_000_000u64);
        assert_eq!(v["cpu_pct"], 42.5);
        assert_eq!(v["ram_used_mb"], 4096);
        assert_eq!(v["ram_total_mb"], 8192);
        assert_eq!(v["net_rx_kbps"], 12.5);
        assert_eq!(v["net_tx_kbps"], 0.0);
    }
}
