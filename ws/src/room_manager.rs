use std::{any, collections::{HashMap, HashSet}, ops::Deref};

use actix_web::body::MessageBody;
use common::types::{ChatUpdateArgs, CreateGameArgs, CreateInMemoryGameArgs, Move, MoveType, MoveUpdateArgs, Player, SendMessageArgs, WebSocketResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

const MAGIC_SQUARE: [[u8 ;3]; 3] = [[4,9,2], [3,5,7], [8,1,6]];

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub tx: Sender<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub players: Vec<Player>,
    pub state: Vec<Vec<Option<MoveType>>>,
    pub moves: Vec<Move>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub players: HashSet<Uuid>,
    pub spectators: HashSet<Uuid>,
    pub game: Option<Game>
}

#[derive(Debug)]
pub struct RoomManager {
    pub clients: HashMap<Uuid, User>,
    pub rooms: HashMap<Uuid, Room>
}


impl RoomManager {
    pub fn new() -> Self {
        Self { 
            clients: HashMap::new(), 
            rooms: HashMap::new() 
        }
    }

    pub fn init_room() -> Room {
        Room {
            players: HashSet::new(),
            spectators: HashSet::new(),
            game: None
        }
    }

    pub fn create_game(&mut self, args: CreateInMemoryGameArgs) -> anyhow::Result<()>{
        let players = args.players.0;
        let initial_state = args.state.0;
        let initial_moves = args.moves.0;

        let game = Game { 
            id: args.game_id, 
            players: players, 
            state: initial_state, 
            moves: initial_moves 
        };

        let room = self.rooms.get_mut(&args.room_id).unwrap();
        room.game = Some(game);

        Ok(())
    }

    pub async fn broadcast_message(&self, args: SendMessageArgs) -> anyhow::Result<()> {
        if let Some(room_members) = self.rooms.get(&args.room_id) {
            let all_members: HashSet<Uuid>  = room_members.players.union(&room_members.spectators).copied().collect();
            for user_id in all_members {
                if let Some(client) = self.clients.get(&user_id) {
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

    pub fn update_game(&mut self, args: MoveUpdateArgs) -> anyhow::Result<&Room> {
        if let Some(room) = self.rooms.get_mut(&args.room_id) {
            if let Some(game) = &mut room.game {
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
        if let Some(room_members) = self.rooms.get(&args.room_id) {
            let all_members: HashSet<Uuid>  = room_members.players.union(&room_members.spectators).copied().collect();
            for user_id in all_members {
                if let Some(client) = self.clients.get(&user_id) {
                    let response = WebSocketResponse {
                        data: common::types::ResponseData::MoveUpdate(MoveUpdateArgs {
                            game_id: args.game_id,
                            room_id: args.room_id,
                            move_type: args.move_type,
                            x_pos: args.x_pos,
                            y_pos: args.y_pos
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

        if magic_numbers.len() < 3 {
            return false;        
        }

        //three sum to check if any combination of these numbers gives 15
        RoomManager::three_sum(magic_numbers)
    }

    fn three_sum(magic_numbers: Vec<u8>) -> bool {
        let mut set = HashSet::<u8>::new();
        let length = magic_numbers.len();
        for i in 0..length {
            for j in i+1..length {
                let diff = 15 - (magic_numbers[i] + magic_numbers[j]) as u8;
                if set.contains(&diff) {
                    return true;
                }
                set.insert(magic_numbers[j]);
            }
        }
        false
    }
}