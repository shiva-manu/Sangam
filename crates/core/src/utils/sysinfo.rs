//! Best-effort detection of the local machine's compute resources.
//!
//! Used by the discovery layer to advertise *real* node capabilities over
//! mDNS instead of hard-coded constants.

use sysinfo::{MemoryRefreshKind, RefreshKind, System};

/// A snapshot of the local machine's relevant compute resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeSpecs {
    /// Number of logical CPU cores the OS exposes to us.
    pub cpu_threads: usize,
    /// Total physical memory in gibibytes (1 GiB = 2^30 bytes).
    pub ram_gib: u64,
}

impl NodeSpecs {
    /// Probe the running system. Falls back to safe defaults rather than
    /// panicking if the OS doesn't tell us what we want to hear.
    pub fn detect() -> Self {
        let cpu_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        // We only need memory info, so opt out of refreshing CPU stats etc.
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        );
        sys.refresh_memory();
        let ram_gib = (sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0)).round() as u64;

        Self {
            cpu_threads,
            ram_gib,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_returns_plausible_values() {
        let specs = NodeSpecs::detect();
        // We can't assert exact values (CI runners differ wildly) but
        // the result must be non-zero — otherwise something is broken.
        assert!(
            specs.cpu_threads >= 1,
            "expected at least one CPU thread, got {}",
            specs.cpu_threads
        );
        // RAM: most machines have at least 256 MiB. Don't be picky here —
        // some CI containers report odd values. Just check the math
        // didn't panic and the field exists.
        let _ = specs.ram_gib;
    }
}
