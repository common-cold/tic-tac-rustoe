import { useAtomValue } from "jotai";
import { UsernameList } from "./UsernameList";
import { roomAtom } from "@/store/atoms";

export function RoomMemberBar() {
    const room = useAtomValue(roomAtom);

    const players = room!.players.map(p => p.username);
    const spectators = ["CommonCold", "Niggatron", "Naruto", "FoxTrot", "Bitcheless", "LmaoDed"];

    return <div className="w-1/6 h-full thinBorderWithLessRadius flex flex-col">
            <div className="h-1/3 border-b-2 border-[#151f28] px-3 py-3 flex flex-col gap-4">
                <div className="font-bold text-[16px] text-left primaryTextColor">
                    Players
                <UsernameList usernames={players} />
                </div>
                
            </div>
            <div className="flex-1 px-3 py-3 flex flex-col gap-1">
                <div className="font-bold text-[16px] text-left primaryTextColor">
                    Spectators
                </div>
                <UsernameList usernames={spectators}/>

            </div>
        
    </div>
}