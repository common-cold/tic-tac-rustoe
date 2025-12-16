import { Move, MoveType, Player, Role } from "./db"

//WS Request types
export type WebSocketMessage =
    | { CreateRoom: CreateRoomArgs }
    | { CreateGame: CreateInMemoryGameArgs }
    | { JoinRoom: JoinRoomArgs }
    | { LeaveRoom: JoinRoomArgs }
    | { SendMessage: SendMessageArgs }
    | { MoveUpdate: MoveUpdateArgs }

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

export type SendMessageArgs = {
    room_id: string,
    message: string
}

export type MoveUpdateArgs = {
    gameId: string,
    roomId: string,
    moveType: MoveType,
    xPos: number,
    yPos: number,
    isXTurn?: boolean
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

