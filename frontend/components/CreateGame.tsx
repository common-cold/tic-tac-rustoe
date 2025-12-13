import { roomAtom, wsAtom } from "@/store/atoms"
import { Game, Room } from "@/types/db";
import { createGame, getRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai"
import { showErrorToast } from "./Homepage";
import { WebSocketMessage } from "@/types/ws";


export function CreateGame() {
    let [room, setRoom] = useAtom(roomAtom);
    let ws = useAtomValue(wsAtom);

    async function handleClick() {
        if (!room) {
            return;
        }

        //fetch latest room form db
        let roomResponse = await getRoom(room.id);
        if (!roomResponse || roomResponse.status != 200) { 
            let data = roomResponse?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }
        let dbRoom = roomResponse?.data as Room;

        //update roomAtom
        setRoom(dbRoom);

        if(dbRoom.players.size < 2) {
            showErrorToast("Not enough players to start the game");
            return;
        }

        let gameResponse = await createGame({
            roomId: dbRoom.id,
            players: dbRoom.players
        });

        if (!gameResponse || gameResponse.status != 200) { 
            let data = gameResponse?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }

        let dbGame = gameResponse.data as Game


        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const msg: WebSocketMessage = {
            CreateGame: {
                game_id: dbGame.id,
                room_id: dbGame.room_id,
                players: dbGame.players,
                state: dbGame.state,
                moves: dbGame.moves
            }
        }  

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));
    }
    
    return <div>
        <button className="rounded-[7px] w-3/4 h-[50px] primaryButton font-bold"
            onClick={handleClick}>
            Start Game
        </button>
    </div>
}