use std::collections::{HashMap, HashSet};

use common::types::{ChatUpdateArgs, SendMessageArgs, WebSocketResponse};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;



pub struct User {
    pub tx: Sender<String>
}

pub struct RoomManager {
    pub clients: HashMap<Uuid, User>,
    pub rooms: HashMap<Uuid, HashSet<Uuid>>
}

impl RoomManager {
    pub fn new() -> Self {
        Self { 
            clients: HashMap::new(), 
            rooms: HashMap::new() 
        }
    }

    pub async fn broadcast_message(&self, args: SendMessageArgs) -> anyhow::Result<()> {
        if let Some(user_ids) = self.rooms.get(&args.room_id) {
            for user_id in user_ids {
                if let Some(client) = self.clients.get(user_id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::ChatUpdate(ChatUpdateArgs {
                            message: args.message.clone()
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }
        }

        Ok(())
    }
}