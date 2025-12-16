use chrono::{Days, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub exp: i64
}

impl Claims {
    pub fn new(sub: Uuid, username: String) -> Self {
        Self { 
            sub: sub, 
            username: username,
            exp: Utc::now().checked_add_days(Days::new(10)).unwrap().timestamp()
        }
    }
}