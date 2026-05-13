//! Task subsystem — definition, execution, and lifecycle tracking.
//!
//! A *task* is a unit of distributed work that one Sangam node delegates to
//! another. This module covers every stage of that journey:
//!
//! 1. **Definition** ([`task`]) — the [`task::Task`] payload and the
//!    [`task::TaskType`] enum that names the supported computation kinds.
//! 2. **Execution** ([`executor`]) — receives an incoming [`task::Task`],
//!    runs the requested computation, and returns a [`result::TaskResult`].
//! 3. **Result** ([`result`]) — the [`result::TaskResult`] payload that a
//!    worker node sends back to the originating node upon completion.
//! 4. **Tracking** ([`tracker`]) — [`tracker::TaskTracker`] maintains an
//!    in-memory record of every task seen by this node, progressing through
//!    the lifecycle: `Queued → Running → Completed | Failed`.
//!
//! The most commonly used tracker types are re-exported at this level for
//! ergonomic access.

pub mod executor;
pub mod result;
pub mod task;
pub mod tracker;

pub use tracker::{StatusCounts, TaskDirection, TaskRecord, TaskStatus, TaskTracker};
