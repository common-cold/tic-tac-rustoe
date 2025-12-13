import { Move, MoveType, Player, Role } from "./db"

//WS Request types
export type WebSocketMessage =
    | { CreateRoom: CreateRoomArgs }
    | { CreateGame: CreateInMemoryGameArgs }
    | { JoinRoom: JoinRoomArgs }
    | { LeaveRoom: JoinRoomArgs }

export type CreateRoomArgs = {
    room_id: string
}

export type CreateInMemoryGameArgs = {
    game_id: string,
    room_id: string,
    players: Player[],
    state: (MoveType | null)[][],
    moves: Move[]
}

export type JoinRoomArgs = {
    room_id: string,
    role: Role
}



//WS Response types
export type WebSocketResponseType = "MoveUpdate" | "Log" | "ChatUpdate";

export interface WebSocketResponseWrapper {
    data: WebSocketResponse;
}

export interface WebSocketResponse {
    type: WebSocketResponseType,
    payload: ResponseData
}

export type ResponseData = 
    | LogArgs


export interface LogArgs {
    message: string,
    isError: boolean
}

