import { Game, Move, MoveType, Player, Role, Room } from "./db"

//WS Request types
export type WebSocketMessage =
    | { CreateRoom: CreateRoomArgs }
    | { CreateGame: CreateInMemoryGameArgs }
    | { JoinRoom: JoinRoomArgs }
    | { LeaveRoom: LeaveRoomArgs }
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
    role: Role,
    symbol?: MoveType
}

export type LeaveRoomArgs = {
    room_id: string,
    game_id?: string | null,
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
export type WebSocketResponseType = "MoveUpdate" | "Log" | "ChatUpdate" | "RoomUpdate" | "StartGame" | "EndGame" | "RoomClose";

export interface WebSocketResponseWrapper {
    data: WebSocketResponse;
}

export interface WebSocketResponse {
    type: WebSocketResponseType,
    payload: ResponseData
}

export type ResponseData = 
    | LogArgs
    | MoveUpdateArgs
    | RoomUpdateArgs
    | EndGameArgs


export interface LogArgs {
    message: string,
    isError: boolean
}

export interface RoomUpdateArgs extends Pick<Room, 'players' | 'spectators' | 'admin'> {}

export interface EndGameArgs {
    isDraw: boolean,
    winner?: string
}