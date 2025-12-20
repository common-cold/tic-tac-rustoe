use actix_web::{HttpResponse, get, post, web};
use common::{auth::JwtClaims, types::{CreateRoom, GetRooms, JoinRoom, Player, Role, Room, Spectator}};
use db::Database;
use serde_json::json;
use page_hunter::paginate_records;
use uuid::Uuid;



#[post("/room")]
pub async fn create_room(db: web::Data<Database>, body: web::Json<CreateRoom>, claims: JwtClaims) -> HttpResponse {
    if body.max_spectators > 8 {
        return HttpResponse::Conflict().json(json!({
            "error": "Cannot have more than 8 spctators"
        }));
    }

    let database = db.get_ref();
    match database.create_room(claims.0.sub, claims.0.username, &body.room_name, &body.max_spectators).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}

#[get("/rooms")]
pub async fn get_rooms_paginated(db: web::Data<Database>, body: web::Json<GetRooms>) -> HttpResponse {  
    let database = db.get_ref();
    match database.get_all_rooms(body.status.clone()).await {
        Ok(rooms) => {
            let mut page: usize = 0;
            let mut limit: usize = 2;
            if body.page.is_some() {
                page = body.page.unwrap();
            }
            if body.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&rooms, page, limit) {
                Ok(p) => HttpResponse::Ok().json(p),

                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "error": e.to_string()
                }))
            }
        }

        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}


#[post("/room/join")]
pub async fn join_room(db: web::Data<Database>, body: web::Json<JoinRoom>, claims: JwtClaims) -> HttpResponse {
    let database = db.get_ref();
    match database.get_room_by_code(&body.room_code).await {
        Ok(mut room) => {
            let room_clone = room.clone();
            if let Err(e) = check_role_capacity(&body.role, &room) {
                return HttpResponse::Conflict().json(json!({
                    "error": e.to_string()
                }));
            }

            match body.role {
                Role::Player => {
                    room.players.push(Player {
                        id: claims.0.sub,
                        username: claims.0.username,
                        symbol: None
                    });
                    if let Err(e) = database.update_room(&room.id, Some(room.status),
                    Some(room.players), Some(room.spectators)).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }
                    
                    //handle user side changes 
                }
                Role::Spectator => {
                    room.spectators.push(Spectator {
                        id: claims.0.sub,
                        username: claims.0.username
                    });
                    if let Err(e) = database.update_room(&room.id, Some(room.status),
                    Some(room.players), Some(room.spectators)).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }

                    //handle user side changes 
                }
            };

            return HttpResponse::Ok().json(room_clone);

        }

        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}


#[post("/room/leave")]
pub async fn leave_room(db: web::Data<Database>, body: web::Json<JoinRoom>, claims: JwtClaims) -> HttpResponse {
    let database = db.get_ref();
    match database.get_room_by_code(&body.room_code).await {
        Ok(mut room) => {
            match body.role {
                Role::Player => {
                    room.players.retain(|s| s.id != claims.0.sub);
                    if let Err(e) = database.update_room(&room.id, Some(room.status),
                    Some(room.players), Some(room.spectators)).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }
                    
                    //handle user side changes 
                }
                Role::Spectator => {
                    room.spectators.retain(|s| s.id != claims.0.sub);
                    if let Err(e) = database.update_room(&room.id, Some(room.status),
                    Some(room.players), Some(room.spectators)).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }

                    //handle user side changes 
                }
            };

            return HttpResponse::Ok().json(json!({
                "message": "Succesfully left the room"
            }));

        }

        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}


#[get("/room/{id}")]
pub async fn get_room(db: web::Data<Database>, path: web::Path<Uuid>) -> HttpResponse {
    let room_id = path.into_inner();
    let database = db.get_ref();

    match database.get_room_by_id(&room_id).await {
        Ok(room) => {
            return HttpResponse::Ok().json(room);
        }
        Err(e) => {
            return HttpResponse::Conflict().json(json!({
                "error": e.to_string()
            }));
        }
    }
}



pub fn check_role_capacity(role: &Role, room: &Room) -> anyhow::Result<()> {
    match role {
        Role::Player => {
            if room.players.len() >= room.max_players as usize {
                return Err(anyhow::anyhow!("Max Player capacity reached"));
            }
        }
        Role::Spectator => {
            if room.spectators.len() >= room.max_spectators as usize {
                return Err(anyhow::anyhow!("Max Spectator capacity reached"));
            }
        }
    }

    Ok(())
}