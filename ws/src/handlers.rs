use std::sync::MutexGuard;

use actix_ws::Session;
use common::types::{CreateRoomArgs, JoinRoomArgs, LeaveRoomArgs, Player, Role, Spectator};
use db::Database;
use sqlx::database;
use uuid::Uuid;

use crate::{room_manager::RoomManager, utils::prepare_log};


pub async fn create_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: CreateRoomArgs, user_id: Uuid, username: String) {
    let mut room = RoomManager::init_room();
    room.players.push(Player {
        id: user_id,
        username: username.clone(),
        symbol: None
    });
    room_manager.rooms.insert(args.room_id, room);
    let log = prepare_log(String::from("Room Created Successfully"), false);
    let _ = session.text(log).await;
}

pub async fn join_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: JoinRoomArgs, user_id: Uuid, username: String) {
    if let None = room_manager.rooms.get(&args.room_id) {
        let log = prepare_log(String::from("Room doesn't exist"), true);
        let _ = session.text(log).await;
        return;
    }
    match args.role {
        Role::Player => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .players
            .push(Player { 
                id: user_id, 
                username: username.clone(), 
                symbol: args.symbol
            });
        },

        Role::Spectator => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .spectators
            .push(Spectator {
                id: user_id,
                username: username.clone()
            });
        }
    };

    let log = prepare_log(String::from("Room Joined Successfully"), false);
    let _ = session.text(log).await;
    
    if let Err (e) = room_manager.broadcast_room_update(&args.room_id, &user_id, &username, true).await {
        let log = prepare_log(format!("Error while updating room: {}", e.to_string()), true);
        let _ = session.text(log).await;
    }
}

pub async fn leave_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: LeaveRoomArgs, user_id: Uuid, username: &String, database: Database) {
    if let None = room_manager.rooms.get(&args.room_id) {
        let log = prepare_log(String::from("Room doesn't exist"), true);
        let _ = session.text(log).await;
        return;
    }
    match args.role {
        Role::Player => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .players
            .retain(|p| p.id != user_id);
        },

        Role::Spectator => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .spectators
            .retain(|p| p.id != user_id);
        }
    };

    let log = prepare_log(String::from("Room Left Successfully"), false);
    let _ = session.text(log).await;

    //Last player left so close the room
    if room_manager.rooms.get(&args.room_id).unwrap().players.len() == 0 {
        let mut session_clone = session.clone();
        tokio::join!(
            async {
                if let Err (e) = room_manager.broadcast_room_close(&args.room_id, &user_id).await {
                    let log = prepare_log(format!("Error while broadcasting room close: {}", e.to_string()), true);
                    let _ = session.text(log).await;
                }
            }, 
            async {
                if let Err(e) = database.update_room(&args.room_id, Some(common::types::RoomStatus::Closed), None, None).await {
                    let log = prepare_log(format!("Error in Db close game update: {:?}", e.to_string()), true);
                    let _ = session_clone.text(log).await;
                }
            }
        );
        room_manager.rooms.remove(&args.room_id).unwrap();
        return;
    }

    if let Err (e) = room_manager.broadcast_room_update(&args.room_id, &user_id, username, false).await {
        let log = prepare_log(format!("Error while updating room: {}", e.to_string()), true);
        let _ = session.text(log).await;
    }

    if let Some(game_id) = args.game_id {
        if let Role::Spectator = args.role {
            return;
        }
        let room = room_manager.rooms.get(&args.room_id).unwrap();
        if let Some(game) = &room.game {
            if game.id == game_id {
                //game ends, other player wins
                let other_player: &Player= game.players.iter().filter(|p| p.id != user_id).collect::<Vec<&Player>>()[0];
                let other_player_clone = other_player.clone();
                let mut session_clone = session.clone();
                tokio::join!(
                    async {
                        if let Err(e) = room_manager.broadcast_end_game(&args.room_id, false, Some(other_player.username.clone()), Some(&user_id)).await {
                            let log = prepare_log(format!("Error in broadcasting end game: {:?}", e.to_string()), true);
                            let _ = session.text(log).await;
                        }  
                    },
                    async {
                        if let Err(e) = database.update_game(&args.game_id.unwrap(), None, None, None, Some(other_player_clone.id), Some(true), Some(chrono::Utc::now().timestamp())).await {
                            let log = prepare_log(format!("Error in Db end game update: {:?}", e.to_string()), true);
                            let _ = session_clone.text(log).await;
                        }
                    }
                );
                let room_mut = room_manager.rooms.get_mut(&args.room_id).unwrap();
                room_mut.game = None;
            } 
        }
    }
}
