use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    Sum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub task_id: String,
    pub task_type: TaskType,
    pub numbers: Vec<i32>,
}
