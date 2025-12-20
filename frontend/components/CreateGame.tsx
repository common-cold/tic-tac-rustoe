import { gameAtom, playerMoveTypeAtom, roomAtom, showGameMenuModalAtom, userAtom, wsAtom } from "@/store/atoms"
import { Game, Player, Room } from "@/types/db";
import { createGame, getRoom } from "@/utils/api";
import { useAtom, useAtomValue, useSetAtom } from "jotai"
import { showErrorToast } from "./Homepage";
import { WebSocketMessage } from "@/types/ws";
import { useRouter } from "next/navigation";


export function CreateGame() {
    let [playerMoveType, setPlayerMoveType] = useAtom(playerMoveTypeAtom);
    let [user, setUser] = useAtom(userAtom);
    let [room, setRoom] = useAtom(roomAtom);
    let ws = useAtomValue(wsAtom);
    let setGame = useSetAtom(gameAtom);
    const [showGameMenuModal, setShowGameMenuModal] = useAtom(showGameMenuModalAtom);
    let router = useRouter();

    async function handleClick() {
        if (!room || !user) {
            return;
        }

        if(room.players.length < 2) {
            showErrorToast("Not enough players to start the game");
            return;
        }

        let playerList = room.players.map(p => p.id);

        let gameResponse = await createGame({
            roomId: room.id,
            players: playerList
        });

        if (!gameResponse || gameResponse.status != 200) { 
            let data = gameResponse?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }

        let dbGame = gameResponse.data as Game

        setGame({
            id: dbGame.id,
            state: dbGame.state,
            moves: dbGame.moves,
            players: dbGame.players
        });

        for (let player of dbGame.players) {
            if (player.id == user.id) {
                setPlayerMoveType(player.symbol);
                break;
            }
        }

        //fetch latest room from db
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

        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const players = reArrangePlayers(dbGame.players, user.id);

        const msg: WebSocketMessage = {
            CreateGame: {
                game_id: dbGame.id,
                room_id: dbGame.room_id,
                players: players,
                state: dbGame.state,
                moves: dbGame.moves
            }
        }  

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));

        setShowGameMenuModal(false);
    }
    
    return <div>
        <button className="rounded-[7px] w-[100px] h-[50px] primaryButton font-bold"
            onClick={handleClick}>
            Start Game
        </button>
    </div>
}

export function reArrangePlayers(players: Player[], userId: string) {
    if (players[0].id == userId) {
        return players
    }
    return [players[1], players[0]]
}