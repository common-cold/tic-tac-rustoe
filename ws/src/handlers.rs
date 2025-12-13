use std::sync::MutexGuard;

use actix_ws::Session;
use common::types::{CreateRoomArgs, JoinRoom, JoinRoomArgs, MoveUpdateArgs, Role};
use uuid::Uuid;

use crate::{room_manager::RoomManager, utils::prepare_log};


pub async fn create_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: CreateRoomArgs, user_id: Uuid) {
    let mut room_members = RoomManager::init_room();
    room_members.players.insert(user_id);
    room_manager.rooms.insert(args.room_id, room_members);
    let log = prepare_log(String::from("Room Created Successfully"), false);
    let _ = session.text(log).await;
}

pub async fn join_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: JoinRoomArgs, user_id: Uuid) {
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
            .insert(user_id)
        },

        Role::Spectator => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .spectators
            .insert(user_id)
        }
    };
    let log = prepare_log(String::from("Room Joined Successfully"), false);
    let _ = session.text(log).await;
}

pub async fn leave_room_handler(room_manager: &mut MutexGuard<'_, RoomManager>, session: &mut Session, args: JoinRoomArgs, user_id: Uuid) {
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
            .remove(&user_id)
        },

        Role::Spectator => {
            room_manager
            .rooms.get_mut(&args.room_id).unwrap()
            .spectators
            .remove(&user_id)
        }
    };
    let log = prepare_log(String::from("Room Left Successfully"), false);
    let _ = session.text(log).await;
}
