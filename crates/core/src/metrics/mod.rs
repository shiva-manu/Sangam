//! Local-machine metrics collection — CPU, RAM, network throughput.
//!
//! The metrics service runs independently of the mesh runtime so the
//! dashboard can display "my own resource usage" even when the user has
//! the runtime stopped. It samples on a fixed cadence (default 1s) and
//! keeps the most recent samples in a ring buffer for chart rendering.
//!
//! Architecture:
//!   * `MetricsSample` — JSON-serializable snapshot the UI consumes.
//!   * `MetricsStore`  — thread-safe ring buffer of samples.
//!   * `run_collector` — async task that drives `MetricsStore` from
//!     `sysinfo` readings.
//!
//! The split exists so tests can exercise the store deterministically
//! without depending on real OS metrics.

pub mod collector;
pub mod sample;
pub mod store;

pub use collector::{DEFAULT_INTERVAL, run_collector};
pub use sample::MetricsSample;
pub use store::{DEFAULT_HISTORY_CAPACITY, MetricsStore};
