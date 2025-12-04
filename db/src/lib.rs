use std::env;

use anyhow::Ok;
use dotenv::dotenv;
use rand::Rng;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::schema::{Room, RoomStatus, User};

pub mod schema;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>
}

static CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url).await?;

        Ok(Self { 
            pool: pool
        })
    }

    pub async fn create_user(&self, email: &String, username: &String, password: &String) -> anyhow::Result<User> {
        let db_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO USERS (email,username, password)
                VALUES ($1, $2, $3)
                RETURNING id, email, username, password, created_at
            "#,
            email,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(db_user)
    }

    pub async fn get_user(&self, username: String, password: String) -> anyhow::Result<User> {
        let db_user = sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    email,
                    username,
                    password,
                    created_at
                FROM USERS
                WHERE username = $1
                AND password = $2
            "#,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(db_user)
    }

    pub async fn create_room(&self, room_name: &String, max_spectators: &i16) -> anyhow::Result<Room> {
        let db_room = sqlx::query_as!(
            Room,
            r#"
                INSERT INTO ROOMS (
                    room_name,
                    room_code,
                    status,
                    players,
                    max_players,
                    spectators,
                    max_spectators
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING 
                    id,
                    room_name,
                    room_code,
                    status AS "status: RoomStatus",
                    players,
                    max_players,
                    spectators,
                    max_spectators,
                    created_at
            "#,
            room_name,
            Database::generate_room_code()?,
            RoomStatus::Open as RoomStatus,
            0,
            2,
            0,
            max_spectators
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(db_room)
    }


    pub async fn get_rooms(&self, status: Option<RoomStatus>) -> anyhow::Result<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
                SELECT 
                    id,
                    room_name,
                    room_code,
                    status AS "status: RoomStatus",
                    players,
                    max_players,
                    spectators,
                    max_spectators,
                    created_at
                FROM ROOMS
                WHERE ($1::room_status IS NULL OR status = $1::room_status)
                ORDER BY created_at DESC
            "#,
            status as Option<RoomStatus>
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rooms)
    }

    pub async fn get_room_by_code(&self, code: &String) -> anyhow::Result<Room> {
        let room = sqlx::query_as!(
            Room,
            r#"
                SELECT 
                    id,
                    room_name,
                    room_code,
                    status AS "status: RoomStatus",
                    players,
                    max_players,
                    spectators,
                    max_spectators,
                    created_at
                FROM ROOMS
                WHERE (room_code = $1)
                ORDER BY created_at DESC
            "#,
            code
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(room)
    }


    pub async fn update_room(&self, room: &Room) -> anyhow::Result<()> {
        sqlx::query!(
            "
                UPDATE ROOMS
                SET
                    status =     $1,
                    players =    $2,
                    spectators = $3
                WHERE id = $4
            ",
            room.status as RoomStatus,
            room.players,
            room.spectators,
            room.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }





    pub fn generate_room_code() -> anyhow::Result<String> {
        let mut generator = rand::rng();
        let code: String = (0..8)
            .map(|_| {
                let index = generator.random_range(0..CHARSET.len());
                CHARSET[index] as char
            })
            .collect();

        Ok(code)
    }
    

}
