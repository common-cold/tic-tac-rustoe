use std::{collections::HashSet, sync::{Arc, Mutex}};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, get, rt, web::{self, Payload}};
use actix_ws::Message;
use common::{auth::JwtClaims, types::{LogArgs, WebSocketMessage, WebSocketResponse}};
use tokio::sync::mpsc;

use crate::{room_manager::{RoomManager, User}, utils::prepare_log};
use common::types::ResponseData::*;

pub mod room_manager;
pub mod utils;

type MutRoomManager = Arc<Mutex<RoomManager>>;

#[get("/ws")]
pub async fn ws_handler(request: HttpRequest, body: Payload, room_manager: web::Data<MutRoomManager>, claims: JwtClaims) -> Result<HttpResponse> {
    let (response, mut session, mut stream) =  actix_ws::handle(&request, body)?;

    let (tx, mut rx) = mpsc::channel(32);

    let user_id = claims.0.sub;
    room_manager
        .lock()
        .unwrap()
        .clients
        .insert(user_id, User {
            tx: tx
        });

    let mut session_clone = session.clone();

    rt::spawn(async move {
        while let Some(message) = stream.recv().await {
            match message.unwrap() {
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }

                Message::Text(data) => {
                    if let Ok(websocket_message) = serde_json::from_slice::<WebSocketMessage>(&data.as_bytes()) {
                        match websocket_message {
                            WebSocketMessage::CreateRoom(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                let mut set = HashSet::new();
                                set.insert(user_id);
                                room_manager.rooms.insert(args.room_id, set);
                                let log = prepare_log(String::from("Room Created Successfully"), false);
                                let _ = session.text(log).await;
                                
                            }   
                            WebSocketMessage::JoinRoom(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                if let None = room_manager.rooms.get(&args.room_id) {
                                    let log = prepare_log(String::from("Room doesn't exist"), true);
                                    let _ = session.text(log).await;
                                    continue;
                                }
                                //make use of args.role
                                room_manager.rooms.get_mut(&args.room_id).unwrap().insert(user_id);
                                let log = prepare_log(String::from("Room Joined Successfully"), false);
                                let _ = session.text(log).await;
                            }

                            WebSocketMessage::LeaveRoom(room_id) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                if let None = room_manager.rooms.get(&room_id) {
                                    let log = prepare_log(String::from("Room doesn't exist"), true);
                                    let _ = session.text(log).await;
                                    continue;
                                }
                                room_manager.rooms.get_mut(&room_id).unwrap().remove(&user_id);
                                let log = prepare_log(String::from("Room Left Successfully"), false);
                                let _ = session.text(log).await;
                            }

                            WebSocketMessage::SendMessage(args) => {
                                let room_manager = room_manager.lock().unwrap();
                                if let Err(e) = room_manager.broadcast_message(args).await {
                                    let log = prepare_log(format!("Error in sending message: {:?}", e.to_string()), true);
                                    let _ = session.text(log).await;
                                }
                            }
                        }
                        
                    }
                }

                _ => {

                }
            }
        }
    });


    rt::spawn(async move {
        while let Some(message) = rx.recv().await {
            let _ = session_clone.text(message).await;
        }
    });

    Ok(response)
}



#[actix_web::main]
pub async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let room_manager = RoomManager::new();
    let mut_room_manager = Arc::new(Mutex::new(room_manager));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mut_room_manager.clone()))
            .service(ws_handler)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await?;

    Ok(())
}