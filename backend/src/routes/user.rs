use actix_web::{HttpResponse, post, web::{self}};
use serde_json::json;
use db::Database;

use crate::routes::Signup;


#[post("/signup")]
pub async fn signup(db: web::Data<Database>, body: web::Json<Signup>) -> HttpResponse {
    let database = db.get_ref();
    match database.create_user(&body.email, &body.username, &body.password).await {
        Ok(user) => HttpResponse::Ok().json(json!({
            "message": "Successfully signed up",
            "userId": user.id
        })),
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}