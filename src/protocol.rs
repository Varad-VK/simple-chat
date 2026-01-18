use std::fmt;

pub type Username = String;

#[derive(Debug, Clone)]
pub enum ClientCommand {
    Join(Username),
    Send(String),
    Leave,
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    Message { from: Username, body: String },
}

pub fn parse_command(line: &str) -> Option<ClientCommand> {
    if let Some(rest) = line.strip_prefix("JOIN ") {
        return Some(ClientCommand::Join(rest.to_string()));
    }
    if let Some(rest) = line.strip_prefix("SEND ") {
        return Some(ClientCommand::Send(rest.to_string()));
    }
    if line == "LEAVE" {
        return Some(ClientCommand::Leave);
    }
    None
}

impl fmt::Display for ServerMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerMessage::Message { from, body } => {
                write!(f, "MSG {from}: {body}")
            }
        }
    }
}
