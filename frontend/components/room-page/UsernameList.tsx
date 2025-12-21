import { roomAtom } from "@/store/atoms";
import { Player } from "@/types/db";
import { useAtomValue } from "jotai";
import { useState } from "react"

export type Member = {
    username: string,
    isAdmin: boolean
}

export type UsernameListProps = {
    members: Member[]
}

export function UsernameList({members}: UsernameListProps) {
    const [isHovered, setIsHovered] = useState<number | null>(null);
    const room = useAtomValue(roomAtom);
    
    return <div className="flex flex-col text-white font-bold mt-3">
        {
            members.map((member, idx) => {
                return <div key={idx} 
                    className={`flex flex-row justify-start gap-3 ${isHovered == idx ? "bg-[#121920]" : ""} px-3 py-2 rounded-[10px]`}
                    onMouseEnter={() => setIsHovered(idx)}
                    onMouseLeave={() => setIsHovered(null)}
                >
                    <div>
                        {idx+1}.
                    </div>
                    <div>
                        {member.username}
                    </div> 
                    {
                        member.isAdmin &&
                        <div className="rounded-[10px] bg-[#ee6677] text-[15px] text-centre px-1">
                            Admin
                        </div>
                    }   
                    
                </div>
            })
        }
    </div>
    
}