use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

use crate::types::MoveType;


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
    pub players: Json<HashSet<Uuid>>,
    pub max_players: i16,
    pub spectators: Json<HashSet<Uuid>>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub id: Uuid,
    pub room_id: Uuid,
    pub players: Json<Vec<Player>>,
    pub state: Json<Vec<Vec<Option<MoveType>>>>,
    pub moves: Json<Vec<Move>>,
    pub winner: Option<Uuid>,
    pub is_completed: bool,
    pub created_at: i64,
    pub completed_at: Option<i64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub symbol: MoveType
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Move {
    pub symbol: MoveType,
    pub x_pos: u8,
    pub y_pos: u8,
    pub timestamp: i64
}