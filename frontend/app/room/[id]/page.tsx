"use client"

import { reArrangePlayers } from "@/components/CreateGame";
import { showErrorToast } from "@/components/Homepage";
import { ChatBar } from "@/components/room-page/ChatBar";
import { RoomMemberBar } from "@/components/room-page/RoomMemberBar";
import { connectSocketAtom, gameAtom, roomAtom, userAtom } from "@/store/atoms";
import { Game as GameType, Room } from "@/types/db";
import { getGame, getRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { use, useEffect, useState } from "react";
import jwt from "jsonwebtoken";
import Board from "@/components/room-page/Board";

export default function RoomPage({params} : 
    {
        params: Promise<{
            id: string
        }>
    }) {
     
    const [loading, setLoading] = useState(true); 
    const [user, setUser] = useAtom(userAtom);   
    const[room, setRoom] = useAtom(roomAtom);
    const [game, setGame] = useAtom(gameAtom);
    const [localUserId, setLocalUserId] = useState(null);
    const [_, connect] = useAtom(connectSocketAtom);
    const { id } = use(params);    
    const router = useRouter();

    async function fetchRoom() {
        let response = await getRoom(id);
        if (!response || response.status != 200) { 
            let data = response?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }
        let dbRoom = response.data as Room;
        setRoom(dbRoom);   
    }

    async function fetchGame() {
        let response = await getGame({
            roomId: id
        });
        if (!response || response.status != 200) { 
            let data = response?.data as any;
            let error = data.error;
            showErrorToast(error);
            return;
        }
        let dbGame = response.data as GameType;
        const players = reArrangePlayers(dbGame.players, localUserId!);
        setGame({
            id: dbGame.id,
            state: dbGame.state,
            moves: dbGame.moves,
            players: players
        })
    }
    
    useEffect(() => {
        async function init() {
            await fetchRoom()
            await fetchGame()
            setLoading(false);
        }

        const token = localStorage.getItem("token");

        if (!token) {
            router.replace("/auth");
        } else {
            const obj = jwt.decode(token) as any;
            setLocalUserId(obj.id);
            setUser({
                id: obj.id,
                username: obj.username
            })
        }

        connect();
        init();

    }, []);

    if (loading) {
        return (
        <div className="flex items-center justify-center h-screen">
            <div className="animate-spin h-8 w-8 border-4 border-[#a7f3d0] border-t-[#c7f284] rounded-full" />
        </div>
        );
    }
    
    return <div className="flex flex-row justify-between w-full h-full px-5 py-5 gap-5">
        <RoomMemberBar/>
        <Board/>
        <ChatBar/>  
    </div>
}