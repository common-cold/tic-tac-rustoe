use std::{collections::HashSet, env, str::FromStr, vec};

use anyhow::Ok;
use common::types::{Game, Move, MoveType, Player, Room, RoomStatus, User};
use dotenv::dotenv;
use rand::Rng;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query_as, types::Json};
use uuid::Uuid;


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

    pub async fn get_user(&self, id: Option<&Uuid>, username: Option<&String>, password: Option<&String>) -> anyhow::Result<User> {
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
                WHERE ($1::uuid IS NULL OR id     = $1)
                AND ($2::text IS NULL OR username = $2)
                AND ($3::text IS NULL OR password = $3)
            "#,
            id,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(db_user)
    }

    pub async fn create_room(&self, user_id: Uuid, room_name: &String, max_spectators: &i16) -> anyhow::Result<Room> {
        let mut players: Vec<Uuid> = Vec::new();
        players.push(user_id);

        let spectators: Vec<Uuid> = Vec::new();

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
                    players as "players!: Json<HashSet<Uuid>>",
                    max_players,
                    spectators as "spectators!: Json<HashSet<Uuid>>",
                    max_spectators,
                    created_at
            "#,
            room_name,
            Database::generate_room_code()?,
            RoomStatus::Open as RoomStatus,
            Json(players) as _,
            2,
            Json(spectators) as _,
            max_spectators
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(db_room)
    }


    pub async fn get_all_rooms(&self, status: Option<RoomStatus>) -> anyhow::Result<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            r#"
                SELECT 
                    id,
                    room_name,
                    room_code,
                    status AS "status: RoomStatus",
                    players as "players!: Json<HashSet<Uuid>>",
                    max_players,
                    spectators as "spectators!: Json<HashSet<Uuid>>",
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
                    players as "players!: Json<HashSet<Uuid>>",
                    max_players,
                    spectators as "spectators!: Json<HashSet<Uuid>>",
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


    pub async fn get_room_by_id(&self, id: &Uuid) -> anyhow::Result<Room> {
        let room = sqlx::query_as!(
            Room,
            r#"
                SELECT 
                    id,
                    room_name,
                    room_code,
                    status AS "status: RoomStatus",
                    players as "players!: Json<HashSet<Uuid>>",
                    max_players,
                    spectators as "spectators!: Json<HashSet<Uuid>>",
                    max_spectators,
                    created_at
                FROM ROOMS
                WHERE (id = $1)
                ORDER BY created_at DESC
            "#,
            id
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
            room.players as _,
            room.spectators as _,
            room.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_game(&self, room_id: &Uuid, players: Vec<Player>) -> anyhow::Result<Game> {
        let initial_state: Vec<Vec<Option<MoveType>>> = vec![vec![None; 3]; 3];
        let initial_moves: Vec<Move> = Vec::new();
        let db_game = sqlx::query_as!(
            Game,
            r#"
                INSERT INTO GAMES (
                    room_id,
                    players,
                    state,
                    moves,
                    winner,
                    is_completed
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING 
                    id,
                    room_id,
                    players as "players!: Json<Vec<Player>>",
                    state as "state!: Json<Vec<Vec<Option<MoveType>>>>",
                    moves as "moves!: Json<Vec<Move>>",
                    winner,
                    is_completed,
                    created_at,
                    completed_at
            "#,
            room_id,
            Json(players) as _,
            Json(initial_state) as _,
            Json(initial_moves) as _,
            None::<Uuid>,
            false
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(db_game)
    }


    pub async fn update_game(
        &self, 
        game_id: &Uuid, 
        players: Option<Json<Vec<Player>>>,
        state: Option<Json<Vec<Vec<Option<MoveType>>>>>,
        moves: Option<Json<Vec<Move>>>,
        winner: Option<Option<Uuid>>,
        is_completed: Option<bool>
    ) -> anyhow::Result<()> {
        let db_game = query_as!(
            Game,
            r#"
                UPDATE GAMES
                SET
                    players = COALESCE($1, players),
                    state = COALESCE($2, state),
                    moves = COALESCE($3, moves),
                    winner = COALESCE($4, winner),
                    is_completed = COALESCE($5, is_completed)
                WHERE id = $6    
            "#,
            players as Option<Json<Vec<Player>>>,
            state as Option<Json<Vec<Vec<Option<MoveType>>>>>,
            moves as Option<Json<Vec<Move>>>,
            winner as Option<Option<Uuid>>,
            is_completed as  Option<bool>,
            game_id
        ).execute(&self.pool)
        .await?;

        Ok(())
    }


    pub async fn get_all_games(&self) -> anyhow::Result<Vec<Game>> {
        let games = sqlx::query_as!(
            Game,
            r#"
            SELECT
                id,
                room_id,
                players as "players!: Json<Vec<Player>>",
                state as "state!: Json<Vec<Vec<Option<MoveType>>>>",
                moves as "moves!: Json<Vec<Move>>",
                winner,
                is_completed,
                created_at,
                completed_at
            FROM GAMES
            ORDER BY created_at DESC 
            "#
        ).fetch_all(&self.pool)
        .await?;

        Ok(games)
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
