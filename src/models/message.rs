use serde::{Deserialize, Serialize};

use crate::tasks::result::TaskResult;
use crate::tasks::task::Task;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    Ping,
    Pong,
    Task(Task),
    Result(TaskResult),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NodeMessage {
    pub node_id: String,
    pub message_type: MessageType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::TaskType;

    fn roundtrip(message: NodeMessage) {
        let json = serde_json::to_string(&message).expect("serialize");
        let back: NodeMessage = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(message, back);
    }

    #[test]
    fn ping_message_roundtrips() {
        roundtrip(NodeMessage {
            node_id: "node-a".to_string(),
            message_type: MessageType::Ping,
        });
    }

    #[test]
    fn task_message_roundtrips() {
        roundtrip(NodeMessage {
            node_id: "node-a".to_string(),
            message_type: MessageType::Task(Task {
                task_id: "t1".to_string(),
                task_type: TaskType::Sum,
                numbers: vec![1, 2, 3],
            }),
        });
    }

    #[test]
    fn result_message_roundtrips() {
        roundtrip(NodeMessage {
            node_id: "node-a".to_string(),
            message_type: MessageType::Result(TaskResult {
                task_id: "t1".to_string(),
                result: 6,
                status: "completed".to_string(),
            }),
        });
    }

    #[test]
    fn malformed_json_fails_to_parse() {
        let bad = r#"{"node_id":"x","message_type":{"InvalidVariant":{}}}"#;
        let parsed: Result<NodeMessage, _> = serde_json::from_str(bad);
        assert!(parsed.is_err(), "expected parse error for unknown variant");
    }

    #[test]
    fn pong_message_roundtrips() {
        roundtrip(NodeMessage {
            node_id: "node-a".to_string(),
            message_type: MessageType::Pong,
        });
    }
}
