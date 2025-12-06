use actix_web::{HttpResponse, get, post, web};
use common::{auth::JwtClaims, types::{CreateRoom, GetRooms, JoinRoom, Role, Room}};
use db::Database;
use serde_json::json;
use page_hunter::paginate_records;



#[post("/room")]
pub async fn create_room(db: web::Data<Database>, body: web::Json<CreateRoom>) -> HttpResponse {
    if body.max_spectators > 8 {
        return HttpResponse::Conflict().json(json!({
            "error": "Cannot have more than 8 spctators"
        }));
    }

    let database = db.get_ref();
    match database.create_room(&body.room_name, &body.max_spectators).await {
        Ok(room) => HttpResponse::Ok().json(json!({
            "message": "Successfully Created Room",
            "userId": room.id
        })),
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}

#[get("/rooms")]
pub async fn get_rooms_paginated(db: web::Data<Database>, body: web::Json<GetRooms>, claims: JwtClaims) -> HttpResponse {
    println!("{:?}", claims);   
    let database = db.get_ref();
    match database.get_rooms(body.status.clone()).await {
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
pub async fn join_room(db: web::Data<Database>, body: web::Json<JoinRoom>) -> HttpResponse {
    let database = db.get_ref();
    match database.get_room_by_code(&body.room_code).await {
        Ok(mut room) => {
            if let Err(e) = check_role_capacity(&body.role, &room) {
                return HttpResponse::Conflict().json(json!({
                    "error": e.to_string()
                }));
            }

            match body.role {
                Role::Player => {
                    room.players += 1;
                    if let Err(e) = database.update_room(&room).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }
                    
                    //handle user side changes 
                }
                Role::Spectator => {
                    room.spectators += 1;
                    if let Err(e) = database.update_room(&room).await {
                        return HttpResponse::Conflict().json(json!({
                            "error": e.to_string()
                        }));
                    }

                    //handle user side changes 
                }
            };

            return HttpResponse::Ok().json(json!({
                "message": "Succesfully joined the room"
            }));

        }

        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
} 



pub fn check_role_capacity(role: &Role, room: &Room) -> anyhow::Result<()> {
    match role {
        Role::Player => {
            if room.players >= room.max_players {
                return Err(anyhow::anyhow!("Max Player capacity reached"));
            }
        }
        Role::Spectator => {
            if room.spectators >= room.max_spectators {
                return Err(anyhow::anyhow!("Max Spectator capacity reached"));
            }
        }
    }

    Ok(())
}