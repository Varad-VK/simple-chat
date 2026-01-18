use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    time::{timeout, Duration},
};

#[tokio::test]
async fn message_is_delivered_to_other_clients_only() {
    // Bind to random port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Run real server accept loop
    tokio::spawn(async move {
        let state = std::sync::Arc::new(tokio::sync::Mutex::new(
            simple_chat::state::ChatState::default(),
        ));

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let state = state.clone();
            tokio::spawn(async move {
                let _ = simple_chat::server::handle_connection_for_test(socket, state).await;
            });
        }
    });

    // Connect client A (varad)
    let a = TcpStream::connect(addr).await.unwrap();
    let (ar, mut aw) = a.into_split();
    let mut ar = BufReader::new(ar);

    // Connect client B (john)
    let b = TcpStream::connect(addr).await.unwrap();
    let (br, mut bw) = b.into_split();
    let mut br = BufReader::new(br);

    aw.write_all(
        b"JOIN varad
",
    )
    .await
    .unwrap();
    bw.write_all(
        b"JOIN john
",
    )
    .await
    .unwrap();

    // varad sends message
    aw.write_all(
        b"SEND hello
",
    )
    .await
    .unwrap();

    // john receives it
    let mut john_buf = String::new();
    br.read_line(&mut john_buf).await.unwrap();

    assert!(john_buf.contains("varad"));
    assert!(john_buf.contains("hello"));

    // varad must NOT receive own message
    let mut varad_buf = String::new();
    let res = timeout(Duration::from_millis(100), ar.read_line(&mut varad_buf)).await;

    assert!(res.is_err());
}
