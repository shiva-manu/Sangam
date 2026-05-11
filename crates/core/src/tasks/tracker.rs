//! Task lifecycle tracker — owns the canonical state for every task that
//! flows through the runtime.
//!
//! A task moves through four states:
//!
//! ```text
//!   record_outbound / record_inbound
//!         │
//!         ▼
//!      Queued ──────► Running ──┬──► Completed
//!                                └──► Failed
//! ```
//!
//! Concurrency: `Arc<TaskTracker>` is shared by the discovery channel,
//! the outbound client (`connect_to_node`), and the inbound server
//! (`handle_connection`). All access funnels through a single
//! `RwLock<HashMap<task_id, TaskRecord>>`. Lookups dominate writes, so
//! `RwLock` is the right primitive.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tokio::sync::RwLock;

/// Lifecycle state for a tracked task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    /// Created locally, not yet sent (or just received, not yet executed).
    Queued,
    /// In flight: outbound — TCP write completed; inbound — execution started.
    Running,
    /// Worker returned a result.
    Completed,
    /// Connection failed, peer hung up, or the executor reported an error.
    Failed,
}

/// Direction relative to *this* node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskDirection {
    /// We sent it to a peer (peer_id = the worker).
    Outbound,
    /// Peer sent it to us (peer_id = the owner).
    Inbound,
}

/// Public, JSON-serializable view of a task's lifecycle.
#[derive(Debug, Clone, Serialize)]
pub struct TaskRecord {
    pub task_id: String,
    /// Human-friendly task type tag, e.g. "Sum". Stored as a string so
    /// adding new TaskType variants doesn't require rev'ing this struct.
    pub task_type: String,
    pub direction: TaskDirection,
    pub status: TaskStatus,
    /// Outbound: the worker we dispatched to.
    /// Inbound: the owner who sent it to us.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_id: Option<String>,
    pub created_at_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at_ms: Option<u64>,
    /// Numeric result for Sum/Product tasks. None until completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<i64>,
    /// Failure reason — populated when `status == Failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Per-status counts for the dashboard's status pills.
#[derive(Debug, Clone, Copy, Default, Serialize, PartialEq, Eq)]
pub struct StatusCounts {
    pub queued: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
}

/// In-memory map of `task_id` → record.
#[derive(Default)]
pub struct TaskTracker {
    inner: RwLock<HashMap<String, TaskRecord>>,
}

impl TaskTracker {
    /// Create an empty tracker. Wrap in `Arc` for sharing across tasks.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a freshly-created outbound task in the `Queued` state.
    pub async fn record_outbound(
        &self,
        task_id: impl Into<String>,
        task_type: impl Into<String>,
        peer_id: Option<String>,
    ) {
        self.upsert_initial(
            task_id.into(),
            task_type.into(),
            TaskDirection::Outbound,
            peer_id,
        )
        .await;
    }

    /// Record an inbound task we just received from a peer.
    pub async fn record_inbound(
        &self,
        task_id: impl Into<String>,
        task_type: impl Into<String>,
        peer_id: Option<String>,
    ) {
        self.upsert_initial(
            task_id.into(),
            task_type.into(),
            TaskDirection::Inbound,
            peer_id,
        )
        .await;
    }

    async fn upsert_initial(
        &self,
        task_id: String,
        task_type: String,
        direction: TaskDirection,
        peer_id: Option<String>,
    ) {
        let now = now_ms();
        let mut inner = self.inner.write().await;
        // Don't overwrite an existing record — repeated discovery events
        // can re-trigger record_outbound for the same task_id, but its
        // lifecycle has already advanced.
        inner.entry(task_id.clone()).or_insert(TaskRecord {
            task_id,
            task_type,
            direction,
            status: TaskStatus::Queued,
            peer_id,
            created_at_ms: now,
            started_at_ms: None,
            completed_at_ms: None,
            result: None,
            error: None,
        });
    }

    /// Move a task to `Running`, stamping `started_at_ms`.
    pub async fn mark_running(&self, task_id: &str) {
        let mut inner = self.inner.write().await;
        if let Some(rec) = inner.get_mut(task_id) {
            // Only advance if we haven't already moved past Running —
            // late mark_running calls (e.g. from a retry) shouldn't
            // clobber a Completed record.
            if matches!(rec.status, TaskStatus::Queued) {
                rec.status = TaskStatus::Running;
                rec.started_at_ms = Some(now_ms());
            }
        }
    }

    /// Move a task to `Completed`, attaching the numeric result.
    pub async fn mark_completed(&self, task_id: &str, result: i64) {
        let mut inner = self.inner.write().await;
        if let Some(rec) = inner.get_mut(task_id) {
            rec.status = TaskStatus::Completed;
            rec.completed_at_ms = Some(now_ms());
            rec.result = Some(result);
            rec.error = None;
        }
    }

    /// Move a task to `Failed`, capturing a free-form reason.
    pub async fn mark_failed(&self, task_id: &str, error: impl Into<String>) {
        let mut inner = self.inner.write().await;
        if let Some(rec) = inner.get_mut(task_id) {
            rec.status = TaskStatus::Failed;
            rec.completed_at_ms = Some(now_ms());
            rec.error = Some(error.into());
        }
    }

    /// Snapshot of every record, **most recent first** (by `created_at_ms`).
    pub async fn list(&self) -> Vec<TaskRecord> {
        let inner = self.inner.read().await;
        let mut out: Vec<TaskRecord> = inner.values().cloned().collect();
        out.sort_by(|a, b| b.created_at_ms.cmp(&a.created_at_ms));
        out
    }

    /// Histogram of statuses across all tracked tasks.
    pub async fn status_counts(&self) -> StatusCounts {
        let inner = self.inner.read().await;
        let mut counts = StatusCounts::default();
        for rec in inner.values() {
            match rec.status {
                TaskStatus::Queued => counts.queued += 1,
                TaskStatus::Running => counts.running += 1,
                TaskStatus::Completed => counts.completed += 1,
                TaskStatus::Failed => counts.failed += 1,
            }
        }
        counts
    }

    /// Number of tracked tasks (any status).
    pub async fn len(&self) -> usize {
        self.inner.read().await.len()
    }

    /// True if no tasks have ever been recorded.
    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }
}

/// Helper: current wall-clock time in milliseconds since the Unix epoch.
fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn record_outbound_starts_in_queued() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", Some("peer-a".into())).await;
        let recs = t.list().await;
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].task_id, "t1");
        assert_eq!(recs[0].task_type, "Sum");
        assert_eq!(recs[0].direction, TaskDirection::Outbound);
        assert_eq!(recs[0].status, TaskStatus::Queued);
        assert_eq!(recs[0].peer_id.as_deref(), Some("peer-a"));
        assert!(recs[0].started_at_ms.is_none());
        assert!(recs[0].completed_at_ms.is_none());
    }

    #[tokio::test]
    async fn record_inbound_carries_owner_peer_id() {
        let t = TaskTracker::new();
        t.record_inbound("t2", "Sum", Some("peer-b".into())).await;
        let recs = t.list().await;
        assert_eq!(recs[0].direction, TaskDirection::Inbound);
        assert_eq!(recs[0].peer_id.as_deref(), Some("peer-b"));
    }

    #[tokio::test]
    async fn mark_running_stamps_started_at() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", None).await;
        t.mark_running("t1").await;
        let r = &t.list().await[0];
        assert_eq!(r.status, TaskStatus::Running);
        assert!(r.started_at_ms.is_some());
    }

    #[tokio::test]
    async fn mark_running_is_idempotent_after_completion() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", None).await;
        t.mark_completed("t1", 42).await;
        // Late mark_running (e.g. retry of an old event) must not undo
        // the completion.
        t.mark_running("t1").await;
        let r = &t.list().await[0];
        assert_eq!(r.status, TaskStatus::Completed);
        assert_eq!(r.result, Some(42));
    }

    #[tokio::test]
    async fn mark_completed_attaches_result_and_clears_error() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", None).await;
        t.mark_failed("t1", "transient blip").await;
        // A retry succeeds; the record should no longer carry the
        // earlier failure reason.
        t.mark_completed("t1", 7).await;
        let r = &t.list().await[0];
        assert_eq!(r.status, TaskStatus::Completed);
        assert_eq!(r.result, Some(7));
        assert!(r.error.is_none());
    }

    #[tokio::test]
    async fn mark_failed_captures_reason() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", None).await;
        t.mark_failed("t1", "connection refused").await;
        let r = &t.list().await[0];
        assert_eq!(r.status, TaskStatus::Failed);
        assert_eq!(r.error.as_deref(), Some("connection refused"));
    }

    #[tokio::test]
    async fn mark_methods_are_no_ops_for_unknown_task_id() {
        let t = TaskTracker::new();
        // Should not panic, should not create phantom records.
        t.mark_running("nope").await;
        t.mark_completed("nope", 0).await;
        t.mark_failed("nope", "no").await;
        assert!(t.is_empty().await);
    }

    #[tokio::test]
    async fn record_is_idempotent_on_repeated_outbound_calls() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", Some("peer-a".into())).await;
        t.mark_running("t1").await;
        // Discovery loops can re-trigger record_outbound for the same id;
        // it must NOT reset state to Queued.
        t.record_outbound("t1", "Sum", Some("peer-a".into())).await;
        assert_eq!(t.list().await[0].status, TaskStatus::Running);
    }

    #[tokio::test]
    async fn list_orders_most_recent_first() {
        let t = TaskTracker::new();
        t.record_outbound("old", "Sum", None).await;
        // tiny pause so created_at_ms differs
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        t.record_outbound("new", "Sum", None).await;
        let ids: Vec<String> = t.list().await.into_iter().map(|r| r.task_id).collect();
        assert_eq!(ids, vec!["new", "old"]);
    }

    #[tokio::test]
    async fn status_counts_aggregates_each_state() {
        let t = TaskTracker::new();
        t.record_outbound("a", "Sum", None).await;
        t.record_outbound("b", "Sum", None).await;
        t.record_outbound("c", "Sum", None).await;
        t.record_outbound("d", "Sum", None).await;
        t.mark_running("b").await;
        t.mark_completed("c", 1).await;
        t.mark_failed("d", "x").await;
        let counts = t.status_counts().await;
        assert_eq!(counts.queued, 1);
        assert_eq!(counts.running, 1);
        assert_eq!(counts.completed, 1);
        assert_eq!(counts.failed, 1);
    }

    #[tokio::test]
    async fn record_serializes_to_expected_json_shape() {
        let t = TaskTracker::new();
        t.record_outbound("t1", "Sum", Some("peer-a".into())).await;
        t.mark_completed("t1", 42).await;
        let r = &t.list().await[0];
        let v = serde_json::to_value(r).unwrap();
        assert_eq!(v["task_id"], "t1");
        assert_eq!(v["task_type"], "Sum");
        assert_eq!(v["direction"], "outbound");
        assert_eq!(v["status"], "completed");
        assert_eq!(v["peer_id"], "peer-a");
        assert_eq!(v["result"], 42);
        // None fields are omitted.
        assert!(v.get("error").is_none());
    }
}
