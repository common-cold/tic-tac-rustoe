import { roomAtom, wsAtom } from "@/store/atoms";
import { WebSocketMessage } from "@/types/ws";
import {leaveRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";


export function LeaveRoom() {
    let ws = useAtomValue(wsAtom);
    const [room, setRoom] = useAtom(roomAtom);

    async function handleLeaveRoom() {
        if (!room) {
            return;
        }

        let response = await leaveRoom({
            roomCode: room?.room_code,
            role: "Spectator"
        });

        if (!response) {
            console.log("Null Response");
            return;
        } else if (response.status != 200) {
            console.log("Error");
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
                room_id: room.id,
                role: "Spectator" 
            }
        }

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));

        setRoom(null);

    }

    return <div>
        <button className="w-30 h-15 bg-blue-500"
            onClick={handleLeaveRoom}>
            Leave Room
        </button>
    </div>
}