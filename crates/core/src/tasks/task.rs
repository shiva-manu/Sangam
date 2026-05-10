use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TaskType {
    Sum,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub task_id: String,
    pub task_type: TaskType,
    pub numbers: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_roundtrips_through_json() {
        let task = Task {
            task_id: "abc-123".to_string(),
            task_type: TaskType::Sum,
            numbers: vec![1, 2, 3],
        };
        let json = serde_json::to_string(&task).expect("serialize");
        let back: Task = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(task, back);
    }
}
