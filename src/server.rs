use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};

use crate::{
    protocol::{parse_command, ClientCommand, ServerMessage},
    state::ChatState,
};

pub async fn run(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let state = Arc::new(Mutex::new(ChatState::default()));

    loop {
        let (socket, _) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let _ = handle_connection(socket, state).await;
        });
    }
}

async fn handle_connection(
    socket: TcpStream,
    state: Arc<Mutex<ChatState>>,
) -> tokio::io::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);
    let mut username = None;
    let mut line = String::new();

    let writer_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let line = format!("{}\n", msg);
            if writer.write_all(line.as_bytes()).await.is_err() {
                break;
            }
        }
    });

    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }

        match parse_command(line.trim()) {
            Some(ClientCommand::Join(name)) => {
                if let Err(err) = state.lock().await.join(name.clone(), tx.clone()) {
                    let _ = tx.send(ServerMessage::Error(err)).await;
                    return Ok(());
                }

                username = Some(name);
            }
            Some(ClientCommand::Send(msg)) => {
                if let Some(ref u) = username {
                    state.lock().await.broadcast(u, &msg).await;
                }
            }
            Some(ClientCommand::Leave) => break,
            None => {}
        }
    }

    if let Some(u) = username {
        state.lock().await.leave(&u);
    }

    writer_task.abort();
    Ok(())
}

pub async fn handle_connection_for_test(
    socket: TcpStream,
    state: Arc<Mutex<ChatState>>,
) -> tokio::io::Result<()> {
    handle_connection(socket, state).await
}
