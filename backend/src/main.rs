use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db::Database;

use crate::routes::{create_game, create_room, get_game, get_games_paginated, get_room, get_rooms_paginated, join_room, leave_room, signin, signup};

pub mod routes;

#[actix_web::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db = Database::new().await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .service(signup)    
            .service(signin)
            .service(create_room)
            .service(get_room)
            .service(get_rooms_paginated)
            .service(join_room)
            .service(leave_room)
            .service(create_game)
            .service(get_game)
            .service(get_games_paginated)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    
    Ok(())
}