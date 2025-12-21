"use client"
import { connectSocketAtom, userAtom, wsAtom } from "@/store/atoms";
import { useAtom } from "jotai";
import { useEffect } from "react"
import toast from "react-hot-toast";
import { useRouter } from "next/navigation";
import jwt from "jsonwebtoken";
import { CreateRoom } from "./CreateRoom";
import { JoinRoom } from "./JoinRoom";

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

    return <main className="h-full relative flex flex-col text-[#e8f5e1] overflow-hidden gridBg">
        <section className="relative flex flex-1 items-start justify-center pt-24 gridBg">
            <div className="flex flex-row justify-center w-full gap-24">
            <div className="w-1/3">
                <CreateRoom />
            </div>

            <div className="w-1/3">
                <JoinRoom />
            </div>
            </div>
        </section>
    </main>
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