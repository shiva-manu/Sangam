//! Task result payload returned by a worker node after executing a task.
//!
//! Once a remote worker finishes (or fails) a [`crate::tasks::task::Task`],
//! it wraps the outcome in a [`TaskResult`] and sends it back to the
//! originating node inside a
//! [`crate::models::message::MessageType::Result`] envelope.

use serde::{Deserialize, Serialize};

/// The outcome of a single task execution, produced by the worker node.
///
/// A `TaskResult` is serialized to JSON and transmitted back to the node
/// that originally dispatched the task, allowing it to record the outcome
/// in its [`crate::tasks::tracker::TaskTracker`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TaskResult {
    /// Unique identifier of the task this result belongs to.
    ///
    /// Must match the [`crate::tasks::task::Task::task_id`] of the
    /// originating task so the receiver can correlate the result with the
    /// correct tracker entry.
    pub task_id: String,

    /// The numeric output produced by executing the task.
    ///
    /// The meaning of this value depends on the
    /// [`crate::tasks::task::TaskType`] that was requested. For
    /// [`crate::tasks::task::TaskType::Sum`], this is the sum of all input
    /// numbers.
    pub result: i32,

    /// Human-readable execution status.
    ///
    /// Known values:
    /// - `"completed"` — the task ran successfully and `result` is valid.
    /// - Any other string — treated as an error description explaining why
    ///   the task could not be completed (e.g. `"integer overflow"`).
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_roundtrips_through_json() {
        let res = TaskResult {
            task_id: "abc-123".to_string(),
            result: 42,
            status: "completed".to_string(),
        };
        let json = serde_json::to_string(&res).expect("serialize");
        let back: TaskResult = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(res, back);
    }
}
