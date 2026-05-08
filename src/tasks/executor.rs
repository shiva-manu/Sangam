use crate::tasks::result::TaskResult;
use crate::tasks::task::{Task, TaskType};

/// Execute a single task and return its result.
///
/// This is intentionally synchronous: every variant currently performs
/// pure CPU work, so making it `async` would only add executor overhead
/// without any real concurrency benefit. The function can be promoted
/// back to `async` later if a variant grows real I/O (e.g. a subprocess
/// for shell tasks).
pub fn execute_task(task: Task) -> TaskResult {
    match task.task_type {
        TaskType::Sum => {
            // Use i64 accumulator + saturating_add so we never panic in
            // debug builds and never silently wrap in release builds.
            let sum: i32 = task
                .numbers
                .iter()
                .copied()
                .fold(0_i64, |acc, n| acc.saturating_add(n as i64))
                .clamp(i32::MIN as i64, i32::MAX as i64) as i32;

            TaskResult {
                task_id: task.task_id,
                result: sum,
                status: "completed".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sum_task(id: &str, numbers: Vec<i32>) -> Task {
        Task {
            task_id: id.to_string(),
            task_type: TaskType::Sum,
            numbers,
        }
    }

    #[test]
    fn sum_of_positive_numbers() {
        let result = execute_task(make_sum_task("t1", vec![1, 2, 3, 4, 5]));
        assert_eq!(result.task_id, "t1");
        assert_eq!(result.result, 15);
        assert_eq!(result.status, "completed");
    }

    #[test]
    fn sum_of_empty_vector_is_zero() {
        let result = execute_task(make_sum_task("t-empty", vec![]));
        assert_eq!(result.result, 0);
        assert_eq!(result.status, "completed");
    }

    #[test]
    fn sum_handles_negative_numbers() {
        let result = execute_task(make_sum_task("t-neg", vec![10, -3, -7, 5]));
        assert_eq!(result.result, 5);
    }

    #[test]
    fn sum_single_element() {
        let result = execute_task(make_sum_task("t-single", vec![42]));
        assert_eq!(result.result, 42);
    }

    #[test]
    fn sum_saturates_on_overflow_instead_of_panicking() {
        // i32::MAX + i32::MAX would panic in debug with plain .iter().sum().
        // Saturating arithmetic must clamp to i32::MAX instead.
        let result = execute_task(make_sum_task("t-overflow", vec![i32::MAX, i32::MAX]));
        assert_eq!(result.result, i32::MAX);
    }

    #[test]
    fn sum_saturates_on_underflow() {
        let result = execute_task(make_sum_task("t-underflow", vec![i32::MIN, i32::MIN]));
        assert_eq!(result.result, i32::MIN);
    }
}
