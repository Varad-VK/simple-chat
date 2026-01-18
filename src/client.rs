use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub async fn run(addr: &str, username: &str) -> tokio::io::Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    writer
        .write_all(format!("JOIN {username}\n").as_bytes())
        .await?;

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut input = String::new();
    let mut server = String::new();

    loop {
        tokio::select! {
        r = reader.read_line(&mut server) => {
        if r? == 0 { break; }
        print!("{server}");
        server.clear();
        }
        _ = stdin.read_line(&mut input) => {
        let cmd = input.trim();
        if cmd == "leave" {
        writer.write_all(b"LEAVE\n").await?;
        break;
        }
        if let Some(msg) = cmd.strip_prefix("send ") {
        writer.write_all(format!("SEND {msg}\n").as_bytes()).await?;
        }
        input.clear();
        }
        }
    }

    Ok(())
}
