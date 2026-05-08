//! Sangam — wireless peer-to-peer distributed compute mesh.
//!
//! This library exposes the core building blocks (discovery, networking,
//! models, tasks, utils) so that integration tests in `tests/` and the
//! `Sangam` binary in `src/main.rs` can share the same code.

#![allow(non_snake_case)]

pub mod discovery;
pub mod models;
pub mod networking;
pub mod tasks;
pub mod utils;
