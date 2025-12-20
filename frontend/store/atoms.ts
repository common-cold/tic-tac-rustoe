import { showErrorToast, showSuccessToast } from "@/components/Homepage";
import { Game, LocalGame, MoveType, Room, User } from "@/types/db";
import { EndGameArgs, LogArgs, MoveUpdateArgs, RoomUpdateArgs, WebSocketResponseWrapper } from "@/types/ws";
import { atom } from "jotai";

export const wsAtom = atom<WebSocket | null>(null);

export const connectSocketAtom = atom(null, (get, set) => {
    if (get(wsAtom) != null) {
        return;
    }

    const token = localStorage.getItem("token");
    let websocket = new WebSocket(`ws://localhost:8081/ws?token=${token}`);

    websocket.onopen = () => console.log("Ws Connected");

    websocket.onmessage = (data) => {
        let responseWrapper: WebSocketResponseWrapper = JSON.parse(data.data) ;
        let response = responseWrapper.data;
        console.log(response);
        switch (response.type) {
            case 'Log':
                let log = response.payload as LogArgs;
                if (log.isError) {
                    showErrorToast(log.message)
                } else {
                    showSuccessToast(log.message)
                }
                break;

            case 'RoomClose':
                set(roomCloseAtom, true);
                break;
                
            case 'StartGame':
                set(gameStartedAtom, true)
                break;
                
            case 'EndGame':
                let endGameUpdate = response.payload as EndGameArgs;
                if (endGameUpdate.isDraw) {
                    set(gameMenuModalTitleAtom, "Match Draw 🤝")
                    set(showGameMenuModalAtom, true);
                } else {
                    set(gameMenuModalTitleAtom, `${endGameUpdate.winner} has won the game 🎉`)
                    set(showGameMenuModalAtom, true);
                }
                set(gameStartedAtom, false);
                break;     

            case 'MoveUpdate':
                let localGame = get(gameAtom);
                if (!localGame) {
                    return;
                }
                let update = response.payload as MoveUpdateArgs;
                localGame.moves.push({
                    symbol: update.moveType,
                    x_pos: update.xPos,
                    y_pos: update.yPos,
                    timestamp: new Date().getTime()/1000
                });
                localGame.state[update.yPos][update.xPos] = update.moveType
                set(gameAtom, {
                    ...localGame,
                    moves: localGame.moves,
                    state: localGame.state
                });
                break;  

            case 'RoomUpdate':
                let room = get(roomAtom);
                if (!room) {
                    return;
                }
                let roomUpdate = response.payload as RoomUpdateArgs;
                room.players = roomUpdate.players;
                room.spectators = roomUpdate.spectators;
                set(roomAtom, {
                    ...room,
                    players: room.players,
                    spectators: room.spectators
                });
                break; 
        }
    };

    websocket.onclose = () => {
        set(wsAtom, null);
    }

    set(wsAtom, websocket);
});

export const roomAtom = atom<Room | null>(null);

export const gameAtom = atom<LocalGame | null>(null);

export const playerMoveTypeAtom = atom<MoveType | null>(null);

export const userAtom = atom<User | null>(null);

export const gameStartedAtom = atom(false);

export const showGameMenuModalAtom = atom(false);

export const gameMenuModalTitleAtom = atom<string | null>(null);

export const refreshGamePageAtom = atom(false);

export const roomCloseAtom = atom(false);