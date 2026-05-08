use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskResult {
    pub task_id: String,
    pub result: i32,
    pub status: String,
}
