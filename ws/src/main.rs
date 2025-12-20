use std::{sync::{Arc, Mutex}};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result, get, rt, web::{self, Payload}};
use actix_ws::Message;
use common::{auth::JwtClaims, types::{WebSocketMessage}};
use db::Database;
use sqlx::types::Json;
use tokio::sync::mpsc;

use crate::{handlers::{create_room_handler, join_room_handler, leave_room_handler}, room_manager::{RoomManager, User}, utils::prepare_log};

pub mod room_manager;
pub mod utils;
pub mod handlers;

type MutRoomManager = Arc<Mutex<RoomManager>>;

#[derive(Clone)]
pub struct WsState {
    pub database: Database,
    pub room_manager: MutRoomManager
}

#[get("/ws")]
pub async fn ws_handler(request: HttpRequest, body: Payload, state: web::Data<WsState>, claims: JwtClaims) -> Result<HttpResponse> {
    let (response, mut session, mut stream) =  actix_ws::handle(&request, body)?;
    let room_manager = state.room_manager.clone();
    let database = state.database.clone();
    
    let (tx, mut rx) = mpsc::channel(32);

    let user_id = claims.0.sub;
    let username = claims.0.username;
    let username_clone = username.clone();

    room_manager
        .lock()
        .unwrap()
        .clients
        .insert(user_id, User {
            username: username_clone,
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
                                create_room_handler(&mut room_manager, &mut session, args, user_id, username.clone()).await;
                            }   

                            WebSocketMessage::CreateGame(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                if let Err(e) = room_manager.create_game(args.clone()) {
                                    let log = prepare_log(format!("Error in Creating in-memory game: {:?}", e.to_string()), true);
                                    let _ = session.text(log).await;
                                }
                                if let Err(e) = room_manager.broadcast_start_game(&args.room_id, &user_id).await {
                                    let log = prepare_log(format!("Error in sending message: {:?}", e.to_string()), true);
                                    let _ = session.text(log).await;
                                };
                            }

                            WebSocketMessage::JoinRoom(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                join_room_handler(&mut room_manager, &mut session, args, user_id, username.clone()).await;
                            }

                            WebSocketMessage::LeaveRoom(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                leave_room_handler(&mut room_manager, &mut session, args, user_id, &username, database.clone()).await;
                            }

                            WebSocketMessage::SendMessage(args) => {
                                let room_manager = room_manager.lock().unwrap();
                                if let Err(e) = room_manager.broadcast_message(args, &user_id).await {
                                    let log = prepare_log(format!("Error in sending message: {:?}", e.to_string()), true);
                                    let _ = session.text(log).await;
                                }
                            }

                            WebSocketMessage::MoveUpdate(args) => {
                                let mut room_manager = room_manager.lock().unwrap();
                                {
                                    if let Err(e) = room_manager.update_game(args) {
                                        let log = prepare_log(format!("Error in Move Update: {:?}", e.to_string()), true);
                                        let _ = session.text(log).await;
                                        return;
                                    }
                                }
                                let room = room_manager.rooms.get(&args.room_id).unwrap();
                                let game = room.game.as_ref().unwrap();
                                let mut session_clone = session.clone();
                                
                                let has_won = room_manager.check_winner(&args.room_id, args.move_type);

                                tokio::join!(
                                    async {
                                        if let Err(e) = room_manager.broadcast_move(args).await {
                                            let log = prepare_log(format!("Error in broadcasting move update: {:?}", e.to_string()), true);
                                            let _ = session.text(log).await;
                                        } 

                                        if has_won {
                                            if let Err(e) = room_manager.broadcast_end_game(&args.room_id, false, Some(username.clone()), None).await {
                                                let log = prepare_log(format!("Error in broadcasting end game: {:?}", e.to_string()), true);
                                                let _ = session.text(log).await;
                                            }
                                        } else if game.moves.len() == 9 {
                                            if let Err(e) = room_manager.broadcast_end_game(&args.room_id, true, None, None).await {
                                                let log = prepare_log(format!("Error in broadcasting end game: {:?}", e.to_string()), true);
                                                let _ = session.text(log).await;
                                            }
                                        }   
                                    },
                                    async {
                                        if has_won {
                                            if let Err(e) = database.update_game(&args.game_id, None, Some(Json(game.state.clone())), Some(Json(game.moves.clone())), Some(user_id), Some(true), Some(chrono::Utc::now().timestamp())).await {
                                                let log = prepare_log(format!("Error in Db end game update: {:?}", e.to_string()), true);
                                                let _ = session_clone.text(log).await;
                                            }
                                        } else if game.moves.len() == 9 {
                                            if let Err(e) = database.update_game(&args.game_id, None, Some(Json(game.state.clone())), Some(Json(game.moves.clone())), None, Some(true), Some(chrono::Utc::now().timestamp())).await {
                                                let log = prepare_log(format!("Error in Db end game update: {:?}", e.to_string()), true);
                                                let _ = session_clone.text(log).await;
                                            }
                                        } else {
                                            if let Err(e) = database.update_game(&args.game_id, None, Some(Json(game.state.clone())), Some(Json(game.moves.clone())), None, None, None).await {
                                                let log = prepare_log(format!("Error in Db move update: {:?}", e.to_string()), true);
                                                let _ = session_clone.text(log).await;
                                            }
                                        }
                                    }
                                );
                                
                                if has_won || game.moves.len() == 9 {
                                    let room_mut = room_manager.rooms.get_mut(&args.room_id).unwrap();
                                    room_mut.game = None;
                                }
                            }

                        }

                        println!("--------------------------------------");
                        {
                            let room_manager = room_manager.lock().unwrap();
                            println!("Rooms = {:?}", room_manager.rooms);
                        }
                        
                    }
                }

                _ => {
                    let log = prepare_log(format!("Invalid Command"), true);
                    let _ = session.text(log).await;
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

    let database = Database::new().await.unwrap();

    let room_manager = RoomManager::sync_db(&database).await.unwrap();
    println!("Rooms = {:?}", room_manager.rooms);

    let mut_room_manager = Arc::new(Mutex::new(room_manager));

    let state = WsState {
        database: database,
        room_manager: mut_room_manager
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(ws_handler)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}