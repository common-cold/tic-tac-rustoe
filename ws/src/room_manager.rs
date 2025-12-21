use std::{collections::{HashMap, HashSet}};

use common::types::{ChatUpdateArgs, CreateInMemoryGameArgs, EndGameArgs, Game, Move, MoveType, MoveUpdateArgs, Player, Room, RoomStatus, RoomUpdateArgs, SendMessageArgs, Spectator, WebSocketResponse};
use db::Database;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::utils::prepare_log;

const MAGIC_SQUARE: [[u8 ;3]; 3] = [[4,9,2], [3,5,7], [8,1,6]];

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub tx: Sender<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalGame {
    pub id: Uuid,
    pub players: Vec<Player>,
    pub state: Vec<Vec<Option<MoveType>>>,
    pub moves: Vec<Move>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalRoom {
    pub players: Vec<Player>,
    pub spectators: Vec<Spectator>,
    pub game: Option<LocalGame>,
    pub admin: Option<Uuid>
}

#[derive(Debug)]
pub struct RoomManager {
    pub clients: HashMap<Uuid, User>,
    pub rooms: HashMap<Uuid, LocalRoom>
}


impl RoomManager {
    pub async fn sync_db(database: &Database) -> anyhow::Result<Self> {
        let clients: HashMap<Uuid, User> =  HashMap::new();
        let mut rooms: HashMap<Uuid, LocalRoom> = HashMap::new();

        let mut room_list: Vec<Room> = Vec::new();
        let mut open_rooms = database.get_all_rooms(Some(RoomStatus::Open)).await?;
        let mut in_progress_rooms = database.get_all_rooms(Some(RoomStatus::InProgress)).await?;
        room_list.append(&mut open_rooms);
        room_list.append(&mut in_progress_rooms);

        let games = database.get_all_games().await?;
        let mut game_map: HashMap<Uuid, Game> = HashMap::new();
        for g in games {
            if g.is_completed {
                continue;
            }
            game_map.insert(g.room_id, g);
        }

        for room in room_list {
            let local_game: Option<LocalGame>;
            if let Some(game) = game_map.get(&room.id) {
                local_game = Some(LocalGame {
                    id: game.id,
                    players: game.players.0.clone(),
                    state: game.state.0.clone(),
                    moves: game.moves.0.clone()
                });
            } else {
                local_game = None;
            }

            let local_room = LocalRoom {
                players: room.players.0.clone(),
                spectators: room.spectators.0.clone(),
                game: local_game,
                admin: room.admin 
            };

            rooms.insert(room.id, local_room);
        };

        Ok(RoomManager {
            clients: clients,
            rooms: rooms
        })
    }

    pub fn init_room() -> LocalRoom {
        LocalRoom {
            players: Vec::new(),
            spectators: Vec::new(),
            game: None,
            admin: None
        }
    }

    pub fn create_game(&mut self, args: CreateInMemoryGameArgs) -> anyhow::Result<()>{
        let players = args.players.0;
        let initial_state = args.state.0;
        let initial_moves = args.moves.0;

        let game = LocalGame { 
            id: args.game_id, 
            players: players, 
            state: initial_state, 
            moves: initial_moves 
        };

        let room = self.rooms.get_mut(&args.room_id).unwrap();
        room.game = Some(game);

        Ok(())
    }

    pub async fn broadcast_message(&self, args: SendMessageArgs, user_id_to_ignore: &Uuid) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(&args.room_id) {
            for player in &room.players {
                if *user_id_to_ignore == player.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&player.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::ChatUpdate(ChatUpdateArgs {
                            message: args.message.clone()
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }

            for spectator in &room.spectators {
                if *user_id_to_ignore == spectator.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&spectator.id) {
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

    pub fn update_game(&mut self, args: MoveUpdateArgs) -> anyhow::Result<&LocalRoom> {
        if let Some(room) = self.rooms.get_mut(&args.room_id) {
            if let Some(game) = &mut room.game {
                if game.moves.len() == 9 {
                    return Err(anyhow::anyhow!("Game is finished"));
                }
                if let Some(_) = game.state[args.y_pos as usize][args.x_pos as usize] {
                    return Err(anyhow::anyhow!("Position is already filled"));
                } 
                game.state[args.y_pos as usize][args.x_pos as usize] = Some(args.move_type);
                game.moves.push(Move {
                    symbol: args.move_type,
                    x_pos: args.x_pos,
                    y_pos: args.y_pos,
                    timestamp: chrono::Utc::now().timestamp()
                });
            }
            return Ok(room);
        }
        return Err(anyhow::anyhow!("Error in updating in-memory game"));
    }

    pub async fn broadcast_move(&self, args: MoveUpdateArgs) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(&args.room_id) {
            let game = room.game.as_ref().unwrap();
            let is_x_turn = game.moves.len() % 2 == 0;
            for player in &room.players {
                if let Some(client) = self.clients.get(&player.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::MoveUpdate(MoveUpdateArgs {
                            game_id: args.game_id,
                            room_id: args.room_id,
                            move_type: args.move_type,
                            x_pos: args.x_pos,
                            y_pos: args.y_pos,
                            is_x_turn: Some(is_x_turn)
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }

            for spectator in &room.spectators {
                if let Some(client) = self.clients.get(&spectator.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::MoveUpdate(MoveUpdateArgs {
                            game_id: args.game_id,
                            room_id: args.room_id,
                            move_type: args.move_type,
                            x_pos: args.x_pos,
                            y_pos: args.y_pos,
                            is_x_turn: Some(is_x_turn)
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn broadcast_room_update(&self, room_id: &Uuid, user_id_to_ignore: &Uuid, username: &String, is_join_update: bool, admin: Option<Uuid>) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(room_id) {
            for player in &room.players {
                if *user_id_to_ignore  == player.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&player.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::RoomUpdate(RoomUpdateArgs {
                            players: room.players.clone(),
                            spectators: room.spectators.clone(),
                            admin: admin
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                    
                    if is_join_update {
                        let log = prepare_log(format!("{} has joined the room!", username), false);
                        let _ = client.tx.send(log).await;
                    }
                    
                }
            }

             for spectator in &room.spectators {
                if *user_id_to_ignore  == spectator.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&spectator.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::RoomUpdate(RoomUpdateArgs {
                            players: room.players.clone(),
                            spectators: room.spectators.clone(),
                            admin: admin
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;

                    if is_join_update {
                        let log = prepare_log(format!("{} has joined the room!", username), false);
                        let _ = client.tx.send(log).await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn broadcast_room_close(&self, room_id: &Uuid, user_id_to_ignore: &Uuid) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(room_id) {
             for spectator in &room.spectators {
                if *user_id_to_ignore  == spectator.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&spectator.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::RoomClose
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn broadcast_start_game(&self, room_id: &Uuid, user_id_to_ignore: &Uuid) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(room_id) {
            for player in &room.players {
                if *user_id_to_ignore  == player.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&player.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::StartGame
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }

            for spectator in &room.spectators {
                if *user_id_to_ignore  == spectator.id {
                    continue;
                }
                if let Some(client) = self.clients.get(&spectator.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::StartGame
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn broadcast_end_game(&self, room_id: &Uuid, is_draw: bool, winner: Option<String>, user_id_to_ignore: Option<&Uuid>) -> anyhow::Result<()> {
        if let Some(room) = self.rooms.get(room_id) {
            for player in &room.players {
                if let Some(id) = user_id_to_ignore {
                    if *id == player.id {
                        continue;
                    }
                }
                if let Some(client) = self.clients.get(&player.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::EndGame(EndGameArgs {
                            is_draw: is_draw,
                            winner: winner.clone()
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }

            for spectator in &room.spectators {
                if let Some(id) = user_id_to_ignore {
                    if *id == spectator.id {
                        continue;
                    }
                }
                if let Some(client) = self.clients.get(&spectator.id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::EndGame(EndGameArgs {
                            is_draw: is_draw,
                            winner: winner.clone()
                        })
                    };
                    let str = serde_json::to_string(&response).unwrap();
                    client.tx.send(str).await?;
                }
            }
        }
        Ok(())
    }

    pub fn check_winner(&self, room_id: &Uuid, move_type: MoveType) -> bool {
        let room = self.rooms.get(&room_id).unwrap();
        let game = room.game.as_ref().unwrap();
        
        //get all magic numbers for this move
        let mut magic_numbers = Vec::<u8>::new();
        for (i, row) in game.state.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if let Some(player_move) = value {
                    match (player_move, move_type) {
                        (MoveType::O, MoveType::O) | (MoveType::X, MoveType::X) => {
                            magic_numbers.push(MAGIC_SQUARE[i][j]);
                        }
                        _ => {

                        }
                    }
                }
            }
        }

        println!("Magik nos: {:?}", magic_numbers);

        if magic_numbers.len() < 3 {
            return false;        
        }

        //three sum to check if any combination of these numbers gives 15
        RoomManager::three_sum(magic_numbers)
    }

    fn three_sum(magic_numbers: Vec<u8>) -> bool {
        let length = magic_numbers.len();
        for i in 0..length {
            let mut set = HashSet::<i8>::new();
            for j in i+1..length {
                let diff = 15 - (magic_numbers[i] + magic_numbers[j]) as i8;
                if set.contains(&diff) {
                    println!("SET: ");
                    println!("{}, {}, {}", magic_numbers[i], magic_numbers[j], diff);
                    return true;
                }
                set.insert(magic_numbers[j] as i8);
            }
        }
        false
    }
}