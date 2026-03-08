"use client"

import { roomAtom, wsAtom } from "@/store/atoms";
import { Room } from "@/types/db";
import { WebSocketMessage } from "@/types/ws";
import { createRoom } from "@/utils/api";
import { useAtomValue, useSetAtom } from "jotai";
import { useState } from "react";
import { LabelInput, LabelInputNumber } from "./LabelInput";
import { useRouter } from "next/navigation";
import { showErrorToast } from "./Homepage";

export function CreateRoom() {
    const [isExpanded, setIsExpanded] = useState(false);
    const [roomName, setRoomName] = useState<string | null>(null);
    const [maxSpectators, setMaxSpectators] = useState<Number>(8);
    const ws = useAtomValue(wsAtom);
    const setRoom = useSetAtom(roomAtom);
    const router = useRouter();

    async function handleCreateRoom() {
        if (!roomName) {
            return;
        }

        let response = await createRoom({
            roomName: roomName,
            maxSpectators: maxSpectators
        });

        if (!response || response.status != 200) { 
            let data = response?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }


        let room = response.data as Room;

        setRoom(room);

        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const msg: WebSocketMessage = {
            CreateRoom: {
                room_id: room.id
            }
        }  

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));

        router.push(`/room/${room.id}`);
        
    }


    return <div className="w-full flex justify-center relative">
        <button className="rounded-[7px] w-3/5 h-[50px] primaryButton font-bold"
            onClick={() => setIsExpanded(prev => !prev)}>
            Create Room
        </button>
        {
            isExpanded &&
            <div className="flex flex-col gap-5 secondaryBg w-3/5 px-2 py-5 rounded-[7px] items-center absolute top-[120%] left-1/2 -translate-x-1/2 z-10">
                <LabelInput label="Room Name" setter={setRoomName}/>
                <LabelInputNumber label="Max Spectators" setter={setMaxSpectators}/>
                <button className="rounded-[7px] w-2/4 h-[30px] mt-3 secondaryButton font-bold"
                    onClick={handleCreateRoom}>
                    Go!
                </button>
            </div>    
        }
    </div>
}