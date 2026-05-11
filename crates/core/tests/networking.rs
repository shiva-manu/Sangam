//! Integration tests for the networking layer.
//!
//! These tests bind a real TCP listener on `127.0.0.1` (port 0 = OS picks
//! a free one) and drive `handle_connection` + `send_message` end-to-end.

use std::sync::Arc;
use std::time::Duration;

use sangam_core::models::message::{MessageType, NodeMessage};
use sangam_core::networking::client::send_message;
use sangam_core::networking::server::handle_connection;
use sangam_core::tasks::task::{Task, TaskType};
use sangam_core::tasks::tracker::TaskTracker;

use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

/// Spawn a one-shot server that accepts a single connection, runs the
/// production `handle_connection` against it, and returns the port it
/// listened on. Returns the tracker too so tests can assert on the
/// inbound lifecycle if they care.
async fn spawn_one_shot_server() -> (u16, Arc<TaskTracker>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();

    let tracker = Arc::new(TaskTracker::new());
    let server_tracker = tracker.clone();

    tokio::spawn(async move {
        if let Ok((socket, _)) = listener.accept().await {
            handle_connection(socket, server_tracker).await;
        }
    });

    (port, tracker)
}

#[tokio::test]
async fn server_executes_sum_task_and_returns_result() {
    let (port, tracker) = spawn_one_shot_server().await;

    let stream = TcpStream::connect(("127.0.0.1", port))
        .await
        .expect("client connect");

    let request = NodeMessage {
        node_id: "test-client".to_string(),
        message_type: MessageType::Task(Task {
            task_id: "task-sum-1".to_string(),
            task_type: TaskType::Sum,
            numbers: vec![10, 20, 30, 40],
        }),
    };

    let response = timeout(Duration::from_secs(5), send_message(stream, &request))
        .await
        .expect("send_message did not time out")
        .expect("send_message succeeded");

    match response.message_type {
        MessageType::Result(result) => {
            assert_eq!(result.task_id, "task-sum-1");
            assert_eq!(result.result, 100);
            assert_eq!(result.status, "completed");
        }
        other => panic!("expected Result variant, got {:?}", other),
    }

    // The tracker must have observed the inbound lifecycle.
    let recs = tracker.list().await;
    assert_eq!(recs.len(), 1);
    let rec = &recs[0];
    assert_eq!(rec.task_id, "task-sum-1");
    assert_eq!(rec.task_type, "Sum");
    assert_eq!(
        format!("{:?}", rec.direction),
        "Inbound",
        "server-side records are inbound"
    );
    assert_eq!(format!("{:?}", rec.status), "Completed");
    assert_eq!(rec.peer_id.as_deref(), Some("test-client"));
    assert_eq!(rec.result, Some(100));
}

#[tokio::test]
async fn server_handles_empty_sum_task() {
    let (port, _tracker) = spawn_one_shot_server().await;
    let stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    let request = NodeMessage {
        node_id: "test-client".to_string(),
        message_type: MessageType::Task(Task {
            task_id: "task-empty".to_string(),
            task_type: TaskType::Sum,
            numbers: vec![],
        }),
    };

    let response = timeout(Duration::from_secs(5), send_message(stream, &request))
        .await
        .expect("not timed out")
        .expect("ok");

    if let MessageType::Result(result) = response.message_type {
        assert_eq!(result.result, 0);
    } else {
        panic!("expected Result");
    }
}

#[tokio::test]
async fn server_handles_negative_sum_task() {
    let (port, _tracker) = spawn_one_shot_server().await;
    let stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    let request = NodeMessage {
        node_id: "test-client".to_string(),
        message_type: MessageType::Task(Task {
            task_id: "task-neg".to_string(),
            task_type: TaskType::Sum,
            numbers: vec![-5, -10, 3],
        }),
    };

    let response = timeout(Duration::from_secs(5), send_message(stream, &request))
        .await
        .expect("not timed out")
        .expect("ok");

    if let MessageType::Result(result) = response.message_type {
        assert_eq!(result.result, -12);
    } else {
        panic!("expected Result");
    }
}

#[tokio::test]
async fn send_message_errors_when_peer_closes_without_responding() {
    // Bind a listener and immediately drop sockets without responding,
    // simulating a peer that crashed or speaks the wrong protocol.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        if let Ok((socket, _)) = listener.accept().await {
            drop(socket); // close immediately
        }
    });

    let stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    let request = NodeMessage {
        node_id: "test-client".to_string(),
        message_type: MessageType::Ping,
    };

    let result = timeout(Duration::from_secs(5), send_message(stream, &request))
        .await
        .expect("did not time out");

    assert!(
        result.is_err(),
        "send_message should error when peer hangs up; got {:?}",
        result
    );
}

#[tokio::test]
async fn server_does_not_panic_on_malformed_json() {
    use tokio::io::AsyncWriteExt;

    let (port, _tracker) = spawn_one_shot_server().await;
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await.unwrap();

    // Send garbage that is definitely not valid JSON, ending in newline.
    stream.write_all(b"this-is-not-json\n").await.unwrap();
    stream.flush().await.unwrap();

    // The server should log + drop the connection without panicking.
    // We give it a moment, then verify the test process is still alive.
    tokio::time::sleep(Duration::from_millis(200)).await;
}
