export type RoomStatus = 
    | "Open"
    | "InProgress"
    | "Closed"

export type Role =
    | "Player"
    | "Spectator"

export type MoveType =
    | "O"
    | "X"

export interface User {
    id: string,
    username: string
}   
    
export interface Room {
    id: string,
    room_name: string,
    room_code: string,
    status: RoomStatus,
    admin?: string,
    players: Player[],
    max_players: Number,
    spectators: Spectator[],
    max_spectators: Number,
    created_at: Number
}

export interface Game {
    id: string,
    room_id: string,
    players: Player[],
    state: (MoveType | null)[][],
    moves: Move[],
    winner: string,
    is_completed: boolean,
    created_at: Number,
    completed_at: Number
}

export interface LocalGame extends Pick<Game, 'id' | 'state' | 'moves' | 'players'> {}


export interface Player {
    id: string,
    username: string,
    symbol: MoveType | null
}

export interface Spectator {
    id: string,
    username: string
}

export interface Move {
    symbol: MoveType,
    x_pos: Number,
    y_pos: Number,
    timestamp: Number
}