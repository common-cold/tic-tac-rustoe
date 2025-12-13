import { useEffect, useRef, useState } from "react";
import { LabelInput } from "./LabelInput";
import { Role } from "@/types/db";


export type DropdownProps = {
    label: string,
    defaultOption: Role,
    options: Role[],
    setter: (option: Role) => void
}

export type OptionProps = {
    option: Role,
    index: number
}

export function Dropdown({label, defaultOption, options, setter}: DropdownProps) {
    const [selectedOption, setSelectedOption] = useState(defaultOption);
    const [isExpanded, setIsExpanded] = useState(false);
    const [hovered, setHovered] = useState<number | null>(null);

    const ref = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        function handleClick(e: MouseEvent) {
            if (!isExpanded && ref.current && !ref.current.contains(e.target as Node)) {
                setIsExpanded(false);
            }
        }

        function handleKeyboard(e: KeyboardEvent) {
            if (!isExpanded && e.key == "Escape") {
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

    return <div 
        ref={ref}
        className="flex flex-col w-3/4 gap-2">
         <div className="font-medium">
            {label}
        </div>
        <div className="flex flex-row justify-between inputStyle items-center w-full relative"
            onClick={() => setIsExpanded(prev => !prev)}>
            <div className="font-medium">
                {selectedOption}
            </div>
            <div
                className="bg-transparent border-none"
            >
                <svg
                    className={`w-[18px] h-[18px] transition-transform ${
                        isExpanded ? "rotate-180" : "rotate-0"
                    } stroke-[white]`}
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    strokeWidth="2"
                >
                    <path strokeLinecap="round" strokeLinejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" />
                </svg>
            </div>
        </div>
        {
            isExpanded &&
            <div className="flex flex-col bg-[#2d2e2e] w-4/6 h-[70px] absolute top-[69%] left-1/2 -translate-x-1/2 z-10">
                {
                    options &&
                    options.map((option, index) => <OptionComponent option={option} index={index}/>)
                }
            </div>
        }
    </div>

    function OptionComponent({option, index}: OptionProps) {
        return <div key={index} className={`w-full ${hovered == index ? "bg-[#1e261e]" : "bg-[#2d2e2e]"} px-2 py-2.5 rounded-[5px]`}
            onMouseEnter={() => setHovered(index)}
            onMouseLeave={() => setHovered(null)}
            onClick={() => {
                setter(option)
                setIsExpanded(false)
                setSelectedOption(option)
            }}
        >  
            {option}
        </div>
    }
}
