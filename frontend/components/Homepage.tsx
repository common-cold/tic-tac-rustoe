"use client"

import { connectSocketAtom, wsAtom } from "@/store/atoms";
import { useAtom } from "jotai";
import { useEffect } from "react"
import { Appbar } from "./Appbar";
import { GameMenu } from "./GameMenu";
import toast from "react-hot-toast";
import { useRouter } from "next/navigation";

export function Homepage() {
    const router = useRouter();
    const [_, connect] = useAtom(connectSocketAtom);

    useEffect(() => {
        const token = localStorage.getItem("token");

        if (!token) {
            router.replace("/auth");
        }

        connect();
    }, []);

    return <div className="flex flex-col gap-y-10 px-5 py-5">
        <Appbar/>
        <div className="flex justify-center">
            <GameMenu/>
        </div>
        
    </div>
}

export function showSuccessToast(message: string) {
    toast.success(
        <div>
            {message}
        </div>,
        { 
            duration: 5000, 
            style: {
            borderRadius: "5px",
            background: "white",
            color: "black",
            fontWeight: "bold"
        }}
    );  
}

export function showErrorToast(message: string) {
    toast.error(
        <div>
            {message}
        </div>,
        { 
            duration: 5000, 
            style: {
            borderRadius: "5px",
            background: "white",
            color: "black",
            fontWeight: "bold"
        }}
    );  
}