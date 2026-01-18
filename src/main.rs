use std::env;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, mode, addr] if mode == "server" => simple_chat::server::run(addr).await,
        [_, mode, addr, user] if mode == "client" => simple_chat::client::run(addr, user).await,
        _ => {
            eprintln!("usage:\n server <addr>\n client <addr> <username>");
            std::process::exit(1);
        }
    }
}
