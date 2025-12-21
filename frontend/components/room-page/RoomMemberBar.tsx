import { useAtomValue } from "jotai";
import { Member, UsernameList } from "./UsernameList";
import { roomAtom } from "@/store/atoms";

export function RoomMemberBar() {
    const room = useAtomValue(roomAtom);

    const players = room!.players.map(p => {
        return {
            username: p.username,
            isAdmin: room!.admin === p.id
        } as Member
    });
    const spectators: Member[] = room!.spectators.map(p => {
        return {
            username: p.username,
            isAdmin: false
        } as Member
    });

    return <div className="w-1/6 h-full thinBorderWithLessRadius flex flex-col">
            <div className="h-1/3 border-b-2 border-[#151f28] px-3 py-3 flex flex-col gap-4">
                <div className="font-bold text-[16px] text-left primaryTextColor">
                    Players
                <UsernameList members={players} />
                </div>
                
            </div>
            <div className="flex-1 px-3 py-3 flex flex-col gap-1">
                <div className="font-bold text-[16px] text-left primaryTextColor">
                    Spectators
                </div>
                <UsernameList members={spectators}/>

            </div>
        
    </div>
}