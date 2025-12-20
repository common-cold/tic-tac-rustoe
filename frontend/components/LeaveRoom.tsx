import { gameAtom, gameStartedAtom, refreshGamePageAtom, roomAtom, showGameMenuModalAtom, userAtom, wsAtom } from "@/store/atoms";
import { Role } from "@/types/db";
import { WebSocketMessage } from "@/types/ws";
import {leaveRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { showErrorToast } from "./Homepage";
import { useRouter } from "next/navigation";


export function LeaveRoom() {
    let ws = useAtomValue(wsAtom);
    const [room, setRoom] = useAtom(roomAtom);
    const [user, setUset] = useAtom(userAtom);
    const [game, setGame] = useAtom(gameAtom);
    const [showGameMenuModal, setShowGameMenuModal] = useAtom(showGameMenuModalAtom);
    const [refreshGamePage, setRefreshGamePage] = useAtom(refreshGamePageAtom);
    const [gameStarted, setGameStarted] = useAtom(gameStartedAtom);
    const router = useRouter();

    async function handleLeaveRoom() {
        if (!room && !user) {
            return;
        }

        let role: Role | null = null;
        //get role of this user
        for (const player of room!.players) {
            if (player.id == user!.id) {
                role = "Player"
                break;
            }
        }

        if (!role) {
            for (const spectator of room!.spectators) {
                if (spectator.id == user!.id) {
                    role = "Spectator"
                    break;
                }
            }
        }

        if (!role) {
            return;
        }

        let response = await leaveRoom({
            roomCode: room!.room_code,
            role: role
        });

        if (!response || response.status != 200) { 
            let data = response?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }

        console.log("Successfully left the room");

        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const msg: WebSocketMessage = {
            LeaveRoom: {
                room_id: room!.id,
                game_id: game ? game.id : null,
                role: role
            }
        }

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));

        setRoom(null);
        setGame(null);
        setShowGameMenuModal(false);
        setGameStarted(false);

        router.replace("/");
    }

    return <div className="flex justify-center">
        <button className="rounded-[7px] w-[100px] h-[50px] primaryButton font-bold"
            onClick={handleLeaveRoom}>
            Leave Room
        </button>
    </div>
}