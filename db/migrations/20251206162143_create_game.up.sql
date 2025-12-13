CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE GAMES (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    room_id UUID NOT NULL,
    players JSONB NOT NULL DEFAULT '[]'::jsonb,
    state JSONB NOT NULL DEFAULT '[]'::jsonb,
    moves JSONB NOT NULL DEFAULT '[]'::jsonb,
    winner UUID,
    is_completed BOOLEAN NOT NULL,
    created_at BIGINT NOT NULL DEFAULT EXTRACT(EPOCH FROM NOW()),
    completed_at BIGINT
);