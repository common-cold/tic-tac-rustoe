"use client"

import { connectSocketAtom, userAtom, wsAtom } from "@/store/atoms";
import { useAtom } from "jotai";
import { useEffect } from "react"
import { Appbar } from "./Appbar";
import { GameMenu } from "./GameMenu";
import toast from "react-hot-toast";
import { useRouter } from "next/navigation";
import jwt from "jsonwebtoken";

export function Homepage() {
    const router = useRouter();
    const [_, connect] = useAtom(connectSocketAtom);
    const [user, setUser] = useAtom(userAtom);

    useEffect(() => {
        const token = localStorage.getItem("token");

        if (!token) {
            router.replace("/auth");
        } else {
            const obj = jwt.decode(token) as any;
            setUser({
                id: obj.sub,
                username: obj.username
            })
        }

        connect();
    }, []);

    return <div className="flex flex-col gap-10">
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