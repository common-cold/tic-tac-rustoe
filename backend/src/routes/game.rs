use actix_web::{HttpResponse, get, post, put, web};
use common::types::{CreateGameArgs, GetGame, GetGames, MoveType, Player, RoomStatus, UpdateGame};
use db::Database;
use page_hunter::paginate_records;
use rand::{rng, seq::SliceRandom};
use serde_json::json;
use sqlx::types::Json;


#[post("/game")]
pub async fn create_game(db: web::Data<Database>, body: web::Json<CreateGameArgs>) -> HttpResponse {
    let database: &Database = db.get_ref();

    let room = match database.get_room_by_id(&body.room_id).await {
        Ok(val) => val,
        Err(e) => {
            return HttpResponse::Conflict().json(json!({
                "error": e.to_string()
            }));
        }
    };

    match room.status {
        RoomStatus::Closed => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Cannot Join Closed room"
            }));
        },
        RoomStatus::InProgress => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Cannot Join InProgress room"
            }));
        },
        _ => {}
    }

    let mut players = Vec::new();
    let mut moves = vec![MoveType::O, MoveType::X];
    moves.shuffle(&mut rng());

    for (index, user_id) in body.players.iter().enumerate() {
        let user = database.get_user(Some(&user_id), None, None).await.unwrap();
        players.push(Player {
            id: user.id,
            username: user.username,
            symbol: Some(moves[index])
        });
    }
    match database.create_game(&body.room_id, players.clone()).await {
        Ok(game) => {
            match database.update_room(&body.room_id, Some(common::types::RoomStatus::InProgress), Some(Json(players.clone())), None, None).await {
                Ok(()) => return HttpResponse::Ok().json(game),

                Err(e) => HttpResponse::Conflict().json(json!({
                    "error": e.to_string()
                }))
            }
            
        }
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}


#[post("/game/fetch")]
pub async fn get_game(db: web::Data<Database>, body: web::Json<GetGame>) -> HttpResponse {
    let databse = db.get_ref();
    match databse.get_game(body.game_id, body.room_id).await {
        Ok(game) => {
            HttpResponse::Ok().json(game)
        }
        Err(e) => {
            HttpResponse::Conflict().json(json!({
                "error": e.to_string()
            }))
        }
    }
}

#[get("/games")]
pub async fn get_games_paginated(db: web::Data<Database>, body: web::Json<GetGames>) -> HttpResponse {
    let database: &Database = db.get_ref();

    match database.get_all_games().await {
        Ok(games) => {
            let mut page: usize = 0;
            let mut limit: usize = 2;
            if body.page.is_some() {
                page = body.page.unwrap();
            }
            if body.limit.is_some() {
                limit = body.limit.unwrap();
            }
            match paginate_records(&games, page, limit) {
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


#[put("/game")]
pub async fn update_game(db: web::Data<Database>, body: web::Json<UpdateGame>) -> HttpResponse {
    let databse = db.get_ref();
    match databse.update_game(&body.game_id, body.players.clone(), body.state.clone(), body.moves.clone(), body.winner, body.is_completed, body.completed_at).await {
        Ok(()) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            HttpResponse::Conflict().json(json!({
                "error": e.to_string()
            }))
        }
    }
}