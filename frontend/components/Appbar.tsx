"use client"

import { useAtomValue } from "jotai";
import { Signout } from "./auth/Signout";
import { userAgent } from "next/server";
import { userAtom } from "@/store/atoms";
import { usePathname } from "next/navigation";


export function Appbar() {
    const user = useAtomValue(userAtom);
    const pathname = usePathname();

    return <div className={`flex justify-between items-center px-5 pt-5`}>
        <div className="primaryTextColor font-satoshi font-extrabold text-5xl">
            TicTacFight
        </div>
        <div className="flex flex-row justify-between gap-5 items-baseline">
            {
                user &&
                <div className="font-bold text-[17px]">
                    Hi, {user.username}     
                </div>
            }
            <Signout/>
        </div>
    </div>
}