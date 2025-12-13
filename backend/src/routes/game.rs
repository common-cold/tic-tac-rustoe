use actix_web::{HttpResponse, get, post, put, web};
use common::types::{CreateGameArgs, GetGames, MoveType, Player, UpdateGame};
use db::Database;
use page_hunter::paginate_records;
use serde_json::json;


#[post("/game")]
pub async fn create_game(db: web::Data<Database>, body: web::Json<CreateGameArgs>) -> HttpResponse {
    let database: &Database = db.get_ref();

    let mut players = Vec::new();
    for (index, user_id) in body.players.iter().enumerate() {
        let move_type;
        if index == 0 {
            move_type = MoveType::O;
        } else {
            move_type = MoveType::X;
        }
        let user = database.get_user(Some(&user_id), None, None).await.unwrap();
        players.push(Player {
            id: user.id,
            username: user.username,
            symbol: move_type
        });
    }
    match database.create_game(&body.room_id, players).await {
        Ok(game) => {
            return HttpResponse::Ok().json(game);
        }
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
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
    match databse.update_game(&body.game_id, body.players.clone(), body.state.clone(), body.moves.clone(), body.winner, body.is_completed).await {
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