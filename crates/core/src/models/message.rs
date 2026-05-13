//! Wire-format message envelope for the Sangam node-to-node protocol.
//!
//! All communication between Sangam nodes flows through two types defined
//! here:
//!
//! - [`MessageType`] — an enum whose variants represent every message that
//!   can be exchanged: liveness probes (`Ping`/`Pong`), task dispatch, and
//!   result delivery.
//! - [`NodeMessage`] — the top-level envelope that pairs a sender identity
//!   with a `MessageType` payload. This is the concrete type that gets
//!   serialized to JSON and written to the TCP stream.
//!
//! Messages are framed as **newline-delimited JSON** (one JSON object per
//! line) so that a `BufReader` can split the stream without needing a
//! length-prefix or other binary framing.

use serde::{Deserialize, Serialize};

use crate::tasks::result::TaskResult;
use crate::tasks::task::Task;

/// All message variants that can be exchanged between two Sangam nodes.
///
/// The enum is serialized by `serde_json` using its default externally-tagged
/// representation, e.g. `{"Ping": null}` or `{"Task": { … }}`.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    /// Liveness probe sent by one node to check whether a peer is reachable.
    ///
    /// The receiving node is expected to reply with a [`MessageType::Pong`].
    Ping,

    /// Acknowledgement sent in response to a [`MessageType::Ping`].
    ///
    /// Receiving a `Pong` confirms that the round-trip to the peer succeeded
    /// and that the peer is alive.
    Pong,

    /// A compute task dispatched by this node to a remote worker node.
    ///
    /// The inner [`Task`] contains all the information the worker needs to
    /// execute the job (task ID, computation type, and input data). The
    /// worker is expected to reply with a [`MessageType::Result`] carrying
    /// the same `task_id`.
    Task(Task),

    /// The outcome of a previously dispatched [`MessageType::Task`].
    ///
    /// Sent by the worker node back to the originator after execution
    /// completes (successfully or not). The inner [`TaskResult`] contains
    /// the task ID, the numeric output, and a status string.
    Result(TaskResult),
}

/// Top-level message envelope transmitted between Sangam nodes over TCP.
///
/// Every message on the wire is a `NodeMessage`. The receiver uses `node_id`
/// to identify which peer sent the message and dispatches on `message_type`
/// to decide how to handle it.
///
/// # Serialization
/// Values are serialized to JSON with `serde_json::to_string` and written to
/// the TCP stream followed by a newline (`\n`). The reader splits on `\n` and
/// deserializes each line independently.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NodeMessage {
    /// The unique identifier of the node that originated this message.
    ///
    /// Typically a UUID v4 string assigned when the node process starts. The
    /// receiver uses this field to correlate messages with entries in the
    /// [`crate::peers::PeerRegistry`] and to route result payloads back to
    /// the correct outstanding request.
    pub node_id: String,

    /// The payload carried by this message.
    ///
    /// Determines both the type of information being conveyed and the action
    /// the receiving node should take. See [`MessageType`] for the full set
    /// of variants.
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
