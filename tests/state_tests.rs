use simple_chat::state::ChatState;
use tokio::sync::mpsc;

#[tokio::test]
async fn broadcast_skips_sender() {
    let mut state = ChatState::default();

    let (tx1, mut rx1) = mpsc::channel(1);
    let (tx2, mut rx2) = mpsc::channel(1);

    state.join("a".into(), tx1).unwrap();
    state.join("b".into(), tx2).unwrap();

    state.broadcast("a", "hello").await;

    assert!(rx1.try_recv().is_err());
    assert!(rx2.try_recv().is_ok());
}
