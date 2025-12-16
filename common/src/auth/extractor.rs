use std::{env, future::{Ready, ready}};

use actix_web::{FromRequest, Result, error::ErrorUnauthorized, web::Query};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsQuery {
    pub token: String
}

#[derive(Debug)]
pub struct JwtClaims(pub Claims);

impl FromRequest for JwtClaims {
    type Error = actix_web::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query_res=  Query::<WsQuery>::from_query(req.query_string());
        let header_res = req.headers().get("Authorization");
        let mut token_option: Option<String> = None;

        if query_res.is_ok() {
            let query = query_res.unwrap();
            token_option = Some(query.token.clone());
        } else if header_res.is_some() {
            let token_header_res = header_res.unwrap().to_str();
            if token_header_res.is_ok() {
                token_option = Some(token_header_res.unwrap().to_string());
            }
        }

        if token_option.is_none() {
            return ready(Err(ErrorUnauthorized("JWT Token is not provided")));
        }

        let token = token_option.unwrap();

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
