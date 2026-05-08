use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TaskResult {
    pub task_id: String,
    pub result: i32,
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
