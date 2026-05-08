use serde::{Deserialize, Serialize};

use crate::tasks::result::TaskResult;
use crate::tasks::task::Task;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Ping,
    Task(Task),
    Result(TaskResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeMessage {
    pub node_id: String,
    pub message_type: MessageType,
}