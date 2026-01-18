use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::protocol::{ServerMessage, Username};

#[derive(Default)]
pub struct ChatState {
    users: HashMap<Username, mpsc::Sender<ServerMessage>>,
}

impl ChatState {
    pub fn join(
        &mut self,
        username: Username,
        tx: mpsc::Sender<ServerMessage>,
    ) -> Result<(), String> {
        if self.users.contains_key(&username) {
            return Err("username already taken".into());
        }
        self.users.insert(username, tx);
        Ok(())
    }

    pub fn leave(&mut self, username: &str) {
        self.users.remove(username);
    }

    pub async fn broadcast(&mut self, from: &str, body: &str) {
        let mut dead = Vec::new();

        for (user, tx) in &self.users {
            if user == from {
                continue;
            }

            if tx
                .send(ServerMessage::Message {
                    from: from.to_string(),
                    body: body.to_string(),
                })
                .await
                .is_err()
            {
                dead.push(user.clone());
            }
        }

        for user in dead {
            self.users.remove(&user);
        }
    }
}
