"use client"

import { reArrangePlayers } from "@/components/CreateGame";
import { showErrorToast, showSuccessToast } from "@/components/Homepage";
import { ChatBar } from "@/components/room-page/ChatBar";
import { RoomMemberBar } from "@/components/room-page/RoomMemberBar";
import { connectSocketAtom, gameAtom, gameMenuModalTitleAtom, gameStartedAtom, playerMoveTypeAtom, refreshGamePageAtom, roomAtom, roomCloseAtom, showGameMenuModalAtom, userAtom, wsAtom } from "@/store/atoms";
import { Game as GameType, Role, Room } from "@/types/db";
import { getGame, getRoom, leaveRoom } from "@/utils/api";
import { useAtom, useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { use, useEffect, useRef, useState } from "react";
import jwt from "jsonwebtoken";
import Board from "@/components/room-page/Board";
import GameModalMenu from "@/components/room-page/GameMenuModal";
import { WebSocketMessage } from "@/types/ws";

export default function RoomPage({params} : 
    {
        params: Promise<{
            id: string
        }>
    }) {
     
    const [loading, setLoading] = useState(true); 
    const [user, setUser] = useAtom(userAtom);   
    const ws = useAtomValue(wsAtom);
    const[room, setRoom] = useAtom(roomAtom);
    const [game, setGame] = useAtom(gameAtom);
    const [playerMoveType, setPlayerMoveType] = useAtom(playerMoveTypeAtom);
    const [localUserId, setLocalUserId] = useState(null);
    const [_, connect] = useAtom(connectSocketAtom);
    const [gameStarted, setGameStarted] = useAtom(gameStartedAtom);
    const [showGameMenuModal, setShowGameMenuModal] = useAtom(showGameMenuModalAtom);
    const gameMenuModalTitle = useAtomValue(gameMenuModalTitleAtom);
    const refreshGamePage = useAtomValue(refreshGamePageAtom);
    const [roomClose, setRoomClose] = useAtom(roomCloseAtom);
    const mountedOnce = useRef(false);
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

        for (let player of dbGame.players) {
            if (player.id == localUserId) {
                setPlayerMoveType(player.symbol);
                break;
            }
        }
    }

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

        router.replace("/");
    }
    
    useEffect(() => {
        if (!gameStartedAtom) {
            return;
        }

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
                id: obj.sub,
                username: obj.username
            })
        }

        connect();
        init();

    }, [gameStarted, refreshGamePage]);
    

    useEffect(() => {
        if (roomClose) {
            showSuccessToast("Room Closed by the admin")
            setRoom(null);
            setGame(null);
            setGameStarted(false);
            
            setRoomClose(false);
            router.replace("/");
        }
        
    }, [roomClose]);

    // useEffect(() => {
    //     console.log("CAME IN USEFFECT");
    //     return () => {
    //         console.log("CAME IN USEFFECT RETURN");
    //         handleLeaveRoom();
    //     }
    // }, []);

    if (loading || !room) {
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
        {
            showGameMenuModal &&
            <GameModalMenu title={gameMenuModalTitle}/>
        }
        
    </div>
}