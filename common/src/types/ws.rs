use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Role;


#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketMessage {
    CreateRoom(CreateRoomArgs),
    JoinRoom(JoinRoomArgs),
    LeaveRoom(Uuid),
    SendMessage(SendMessageArgs)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomArgs {
    pub room_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRoomArgs {
    pub room_id: Uuid,
    pub role: Role
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageArgs {
    pub room_id: Uuid,
    pub message: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub data: ResponseData
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseData {
    MoveUpdate(MoveUpdateArgs),
    Log(LogArgs),
    ChatUpdate(ChatUpdateArgs)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveType {
    O,
    X
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveUpdateArgs {
    #[serde(rename="moveType")]
    pub move_type: MoveType,

    #[serde(rename="xPos")]
    pub x_pos: u8,

    #[serde(rename="yPos")]
    pub y_pos: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogArgs {
    pub message: String,

    #[serde(rename="isError")]
    pub is_error: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatUpdateArgs {
    pub message: String
}


