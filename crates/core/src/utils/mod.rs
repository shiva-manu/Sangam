//! Miscellaneous utility helpers used across the Sangam runtime.
//!
//! This module groups small, self-contained helpers that do not belong to any
//! single subsystem:
//!
//! - [`banner`] — prints the Sangam startup banner (project name + version)
//!   to stdout when the node first launches.
//! - [`sysinfo`] — detects and surfaces hardware specifications of the local
//!   machine (CPU count, available memory, etc.) so the node can advertise
//!   its compute capacity to peers during discovery.

pub mod banner;
pub mod sysinfo;
