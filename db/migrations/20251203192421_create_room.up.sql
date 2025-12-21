CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE ROOM_STATUS AS ENUM('Open', 'InProgress', 'Closed');

CREATE TABLE ROOMS (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_name TEXT NOT NULL,
    room_code TEXT UNIQUE NOT NULL,
    status ROOM_STATUS NOT NULL,
    admin UUID DEFAULT NULL,
    players JSONB NOT NULL DEFAULT '[]'::jsonb,
    max_players SMALLINT NOT NULL,
    spectators JSONB NOT NULL DEFAULT '[]'::jsonb,
    max_spectators SMALLINT NOT NULL CHECK (max_spectators <= 8),
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW())
)