import { Role } from "./db"

export interface SignUp {
    email: string,
    username: string,
    password: string
}

export interface SignIn {
    username: string,
    password: string
}

export interface CreateRoom {
    roomName: string,
    maxSpectators: Number
}

export interface JoinRoom {
    roomCode: string,
    role: Role
}

export interface CreateGame {
    roomId: string,
    players: string[]
}

export interface GetGame {
    gameId?: string,
    roomId: string
}