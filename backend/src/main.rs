use actix_web::{App, HttpServer, web};
use db::Database;

use crate::routes::{create_room, get_rooms_paginated, join_room, signin, signup};

pub mod routes;

#[actix_web::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let db = Database::new().await?;

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db.clone()))
        .service(signup)    
        .service(signin)
        .service(create_room)
        .service(get_rooms_paginated)
        .service(join_room)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    
    Ok(())
}