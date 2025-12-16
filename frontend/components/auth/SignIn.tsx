"use client"

import { AuthBoxInputs } from "@/components/auth/AuthBox";
import { showErrorToast } from "@/components/Homepage";
import { signIn } from "@/utils/api";
import { useRouter } from "next/navigation";
import { useState } from "react";
import jwt from "jsonwebtoken";
import { useAtom } from "jotai";
import { userAtom } from "@/store/atoms";

export default function SignIn() {
    const [user, setUser] = useAtom(userAtom);
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const router = useRouter();

    async function handleSignIn() {
        const response = await signIn({
            username: username,
            password: password
        });
        if (!response || response.status != 200) {
            showErrorToast("Unable to Signup");
        } else if (response.status === 200) {
            const data = response.data as any;
            const token = data.token;
            localStorage.setItem("token", token);
            const obj = jwt.decode(token) as any;
            setUser({
                id: obj.sub,
                username: obj.username
            });
            router.push("/");
        }    
    } 

    return <>
        <AuthBoxInputs 
        name="Sign In" 
        argsTupleArray=
            {
                [
                    ["Username", setUsername],
                    ["Password", setPassword]
                ]
            } 
        handleSubmit={handleSignIn}
        isSignUp={true}
    />
    </>
}