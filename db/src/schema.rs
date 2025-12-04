use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: i64
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "room_status")]
#[sqlx(rename_all = "PascalCase")]
pub enum RoomStatus {
    Open,
    InProgress,
    Closed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: Uuid,
    pub room_name: String,
    pub room_code: String,
    pub status: RoomStatus,
    pub players: i16,
    pub max_players: i16,
    pub spectators: i16,
    pub max_spectators: i16,
    pub created_at: i64
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "PascalCase")]
pub enum Role {
    Player,
    Spectator
}