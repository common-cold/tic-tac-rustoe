import {gameAtom, roomAtom, wsAtom } from "@/store/atoms";
import { MoveType } from "@/types/db";
import { WebSocketMessage } from "@/types/ws";
import { useAtom, useAtomValue } from "jotai";
import { useState } from "react";
import { CreateGame } from "../CreateGame";


type CellProps = {
    value: MoveType | null,
    handleOnClick: () => void,
    borderClass: string
}

type PlayerHeaderProps = {
    hasTurn: string
}

type SymbolComponentProps = {
    moveType: MoveType
}

function SymbolComponent({moveType}: SymbolComponentProps) {
    return <div className={`font-bold ${moveType == "X" ? "text-[#ee6677]" : "text-[#18bc9c]"}`}>
        {moveType}
    </div>
}

function PlayerHeader({hasTurn} : PlayerHeaderProps) {
    const room = useAtomValue(roomAtom);
    
    return <div className="flex flex-row gap-5 justify-center items-baseline">
        <SymbolComponent moveType={room!.players[0].symbol!}/>
        <div 
            key={room!.players[0].id}
            className={`${hasTurn == room!.players[0].id ? "secondaryButton" : "text-white"} px-2 py-2 rounded-[7px]`}>
            {room!.players[0].username}
        </div>
        <div> 
            :
        </div>
        {
            room!.players.length == 2 &&
            <>
                <div 
                    key={room!.players[1].id}
                    className={`${hasTurn == room!.players[1].id ? "secondaryButton" : "text-white"} px-2 py-2 rounded-[7px]`}>
                    {room!.players[1].username}
                </div>
                <SymbolComponent moveType={room!.players[1].symbol!}/>
            </>
        }
    </div>
}

function Cell({value, handleOnClick, borderClass}: CellProps) {

    return <div className={`flex ${borderClass} 
        w-35 h-35 text-center text-[100px] justify-center items-center ${value == "X" ? "text-[#ee6677]" : "text-[#18bc9c]"}`}
        onClick={handleOnClick}>
        {value}
    </div>
}

export default function Board() {
    const ws = useAtomValue(wsAtom);
    let [game, setGame] = useAtom(gameAtom)
    const room = useAtomValue(roomAtom);
    let hasTurn;
    let isXTurn;
    if (game) {
        isXTurn = game!.moves.length % 2 == 0; 
        for (const player of game!.players) {
            if (isXTurn) {
                if (player.symbol == "X") {
                    hasTurn = player.id;
                    break;
                }
            } else {
                if (player.symbol == "O") {
                    hasTurn = player.id;
                    break;
                }
            }
        }
    }
    

    async function handleClick(xPos: number, yPos: number) {
        if (!game || !room) {
            console.log("NO GAME");
            return;
        }

        const move: MoveType = isXTurn! ? "X" : "O";

        game.state[yPos][xPos] = move;
        game.moves.push({
            symbol: move,
            x_pos: xPos,
            y_pos: yPos,
            timestamp: new Date().getTime()/1000
        })

        setGame({
            ...game,
            state: game.state,
            moves: game.moves
        });

        //send move update to ws
        if (!ws) {
            console.log("WS is null");
            return;
        }

        if (ws.readyState !== WebSocket.OPEN) {
            console.log("WS not open:", ws.readyState);
            return;
        }

        const msg: WebSocketMessage = {
            MoveUpdate: {
                gameId: game.id,
                roomId: room.id,
                moveType: move,
                xPos: xPos,
                yPos: yPos,
            }
        }  

        console.log(JSON.stringify(msg));

        ws?.send(JSON.stringify(msg));
    }

    function getCellValue(xPos: number, yPos: number) {
        if (!game) {
            console.log("NO CELL VALUE")
            return null;
        }
        console.log("VALUE: " + game.state[yPos][xPos]);
        return game.state[yPos][xPos];
    }


    return <div className="flex flex-col flex-1 h-full thinBorderWithLessRadius justify-center items-center">
        <div className="flex flex-row border-b-2 border-[#151f28] px-3 py-3 justify-center gap-2 items-center w-full">
            {
                !game
                ?
                <CreateGame/>
                :
                <PlayerHeader hasTurn={hasTurn!}/>
            }
        </div>
        
        <div className="flex justify-center items-center w-full h-full">
            <div className="flex flex-col">
                <div className="flex flex-row">
                    <Cell value={getCellValue(0,0)} handleOnClick={async () => handleClick(0,0)} borderClass="boardBorderA"/>
                    <Cell value={getCellValue(1,0)} handleOnClick={async () => handleClick(1,0)} borderClass="boardBorderA"/>
                    <Cell value={getCellValue(2,0)} handleOnClick={async () => handleClick(2,0)} borderClass=""/>
                </div>
                <div className="flex flex-row">
                    <Cell value={getCellValue(0,1)} handleOnClick={async () => handleClick(0,1)} borderClass="boardBorderB"/>
                    <Cell value={getCellValue(1,1)} handleOnClick={async () => handleClick(1,1)} borderClass="boardBorderB"/>
                    <Cell value={getCellValue(2,1)} handleOnClick={async () => handleClick(2,1)} borderClass="boardBorderC"/>
                </div>
                <div className="flex flex-row">
                    <Cell value={getCellValue(0,2)} handleOnClick={async () => handleClick(0,2)} borderClass="boardBorderB"/>
                    <Cell value={getCellValue(1,2)} handleOnClick={async () => handleClick(1,2)} borderClass="boardBorderB"/>
                    <Cell value={getCellValue(2,2)} handleOnClick={async () => handleClick(2,2)} borderClass="boardBorderC"/>
                </div>
            </div>
        </div>
    </div>    
}