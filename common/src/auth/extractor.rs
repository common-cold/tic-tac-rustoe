use std::{env, future::{Ready, ready}};

use actix_web::{FromRequest, Result, error::ErrorUnauthorized};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::auth::Claims;



#[derive(Debug)]
pub struct JwtClaims(pub Claims);

impl FromRequest for JwtClaims {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(token_header) = req.headers().get("Authorization") {
            if let Ok(token) = token_header.to_str() {
                let jwt_secret = env::var("JWT_SECRET");
                if jwt_secret.is_err() {
                    return ready(Err(ErrorUnauthorized("Error loading JWT secret key")));
                }

                let decoded = decode::<Claims>(
                    token, 
                    &DecodingKey::from_secret(jwt_secret.unwrap().as_bytes()), 
                    &Validation::default()
                );

                match decoded {
                    Ok(token_data) => {
                        //check if token has expired
                        if chrono::Utc::now().timestamp() > token_data.claims.exp {
                            return ready(Err(ErrorUnauthorized("Token has expired, please sign in again")))
                        }

                        return ready(Ok(JwtClaims(token_data.claims)));
                    } 

                    Err(_) => {
                        return ready(Err(ErrorUnauthorized("Invalid JWT token")));
                    }
                }
            }
        }
        ready(Err(ErrorUnauthorized("Authorization field missing or invalid")))
    } 
}
