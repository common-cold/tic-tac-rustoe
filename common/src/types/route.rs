use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

use crate::types::{Move, MoveType, Player, Role, RoomStatus, Spectator};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signup {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signin {
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
pub struct CreateGameArgs {
    #[serde(rename = "roomId")]
    pub room_id: Uuid,
    pub players: Vec<Uuid>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRooms {
    pub status: Option<RoomStatus>,
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateRoom {
    #[serde(rename = "roomId")]
    pub room_id: Uuid, 
    pub status: Option<RoomStatus>, 
    pub players: Option<Json<Vec<Player>>>, 
    pub spectators: Option<Json<Vec<Spectator>>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGame {
    #[serde(rename = "gameId")]
    pub game_id: Option<Uuid>,
    #[serde(rename = "roomId")]
    pub room_id: Option<Uuid>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetGames {
    pub page: Option<usize>,
    pub limit: Option<usize>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRoom {
    #[serde(rename = "roomCode")]
    pub room_code: String,
    pub role: Role
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateGame {
    #[serde(rename = "gameId")]
    pub game_id: Uuid,
    pub players: Option<Json<Vec<Player>>>,
    pub state: Option<Json<Vec<Vec<Option<MoveType>>>>>,
    pub moves: Option<Json<Vec<Move>>>,
    pub winner: Option<Uuid>,

    #[serde(rename = "isCompleted")]
    pub is_completed: Option<bool>,

    #[serde(rename = "completedAt")]
    pub completed_at: Option<i64>
}