import { useState } from "react"

export type UsernameListProps = {
    usernames: string[]
}

export function UsernameList({usernames}: UsernameListProps) {
    const [isHovered, setIsHovered] = useState<number | null>(null);
    
    return <div className="flex flex-col text-white font-bold mt-3">
        {
            usernames.map((name, idx) => {
                return <div key={idx} 
                    className={`flex flex-row justify-start gap-3 ${isHovered == idx ? "bg-[#121920]" : ""} px-3 py-2 rounded-[10px]`}
                    onMouseEnter={() => setIsHovered(idx)}
                    onMouseLeave={() => setIsHovered(null)}
                >
                    <div>
                        {idx+1}.
                    </div>
                    <div>
                        {name}
                    </div>    
                </div>
            })
        }
    </div>
    
}