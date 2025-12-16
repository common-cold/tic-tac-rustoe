use std::env;

use actix_web::{HttpResponse, post, web::{self}};
use common::{auth::Claims, types::{MoveType, Player, Signin, Signup, CreateGameArgs}};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use db::Database;


#[post("/signup")]
pub async fn signup(db: web::Data<Database>, body: web::Json<Signup>) -> HttpResponse {
    let database = db.get_ref();
    match database.create_user(&body.email, &body.username, &body.password).await {
        Ok(user) => {
            let jwt_secret = env::var("JWT_SECRET");
            if jwt_secret.is_err() {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Error loading JWT secret key"
                }));
            }

            let token = jsonwebtoken::encode(&Header::default(), &Claims::new(user.id, user.username), &EncodingKey::from_secret(jwt_secret.unwrap().as_bytes()));
            if token.is_err() {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Error creating JWT key"
                }));
            }
            HttpResponse::Ok().json(json!({
                "message": "Successfully signed up",
                "token": token.unwrap()
            }))
        },
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}

#[post("/signin")]
pub async fn signin(db: web::Data<Database>, body: web::Json<Signin>) -> HttpResponse {
    let database: &Database = db.get_ref();
    match database.get_user(None, Some(&body.username), Some(&body.password)).await {
        Ok(user) => {
            let jwt_secret = env::var("JWT_SECRET");
            if jwt_secret.is_err() {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Error loading JWT secret key"
                }));
            }

            let token = jsonwebtoken::encode(&Header::default(), &Claims::new(user.id, user.username), &EncodingKey::from_secret(jwt_secret.unwrap().as_bytes()));
            if token.is_err() {
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Error creating JWT key"
                }));
            }
            return HttpResponse::Ok().json(json!({
                "message": "Signed In Successfully",
                "token": token.unwrap()
            }));
        }
        Err(e) => HttpResponse::Conflict().json(json!({
            "error": e.to_string()
        }))
    }
}
