"use client"

import { roomAtom, wsAtom } from "@/store/atoms";
import { Role, Room } from "@/types/db";
import { WebSocketMessage } from "@/types/ws";
import { joinRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { useEffect, useRef, useState } from "react";
import { LabelInput } from "./LabelInput";
import { Dropdown } from "./Dropdown";
import { useRouter } from "next/navigation";
import { showErrorToast } from "./Homepage";


export function JoinRoom() {
    const [isExpanded, setIsExpanded] = useState(false);
    const [roomCode, setRoomCode] = useState<string | null>(null);
    const [role, setRole] = useState<Role | null>("Spectator");
    let ws = useAtomValue(wsAtom);
    let [room, setRoom] = useAtom(roomAtom);
    const router = useRouter();

    const ref = useRef<HTMLDivElement>(null);
    const isExpandedRef = useRef(isExpanded);

    useEffect(() => {
        isExpandedRef.current = isExpanded
    }, [isExpanded]);

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (isExpandedRef.current && ref.current && !ref.current.contains(e.target as Node)) {
                setIsExpanded(false);
            }
        }

        function handleKeyboard(e: KeyboardEvent) {
            if (isExpandedRef.current && e.key == "Escape") {
                setIsExpanded(false);
            }
        }

        window.addEventListener("mousedown", handleClick);
        window.addEventListener("keydown", handleKeyboard);

        return () => {
            document.removeEventListener("mousedown", handleClick);
            document.removeEventListener("keydown", handleKeyboard);
        }
    }, [])

    async function handleJoinRoom() {
        if (!roomCode || !role) {
            return;
        }

        let response = await joinRoom({
            roomCode: roomCode,
            role: role
        });

        if (!response || response.status != 200) { 
            let data = response?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }

        const dbRoom = response.data as Room;
        setRoom(dbRoom);

        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const msg: WebSocketMessage = {
            JoinRoom: {
                room_id: dbRoom.id,
                role: role
            }
        }  

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));

        router.push(`/room/${dbRoom.id}`);
    }

    return <div 
        ref={ref}
        className="w-full flex flex-col gap-5 relative">
        <div className="w-full flex justify-center">
            <button className="rounded-[7px] w-3/5 h-[50px] primaryButton font-bold"
                onClick={() => setIsExpanded(prev => !prev)}>
                Join Room
            </button>
        </div>
        {
            isExpanded &&
            <div className="flex flex-col gap-5 secondaryBg w-3/5 px-2 py-5 rounded-[7px] items-center absolute top-[120%] left-1/2 -translate-x-1/2 z-10">
                <LabelInput label="Room Code" setter={setRoomCode}/>
                <Dropdown label="Role" defaultOption={role!} options={["Spectator", "Player"]} setter={setRole}/>
                <button className="rounded-[7px] w-2/4 h-[30px] mt-3 secondaryButton font-bold"
                    onClick={handleJoinRoom}>
                    Go!
                </button>
            </div>    
        }
    </div>
    
}