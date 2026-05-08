use crate::tasks::result::TaskResult;
use crate::tasks::task::{Task, TaskType};

pub async fn execute_task(task: Task) -> TaskResult {
    match task.task_type {
        TaskType::Sum => {
            let sum: i32 = task.numbers.iter().sum();

            TaskResult {
                task_id: task.task_id,
                result: sum,
                status: "completed".to_string(),
            }
        }
    }
}
