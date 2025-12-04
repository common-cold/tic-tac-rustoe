use db::schema::{Role, RoomStatus};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signup {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRoom {
    #[serde(rename = "roomName")]
    pub room_name: String,
    #[serde(rename = "maxSpectators")]
    pub max_spectators: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRooms {
    pub status: Option<RoomStatus>,
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRoom {
    pub room_code: String,
    pub role: Role
}