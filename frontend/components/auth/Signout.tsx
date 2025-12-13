"use client"

import { useRouter } from "next/navigation";

export function Signout() {
    const router = useRouter();
    return  <div onClick={() => {
        localStorage.removeItem("token");
        router.replace("/auth");
    }} 
        className="flex w-20 h-10 rounded-[7px] cursor-pointer secondaryButton font-bold items-center justify-center">
        Signout
    </div>

}