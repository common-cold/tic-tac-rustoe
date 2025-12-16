"use client"

import { AuthBoxInputs } from "@/components/auth/AuthBox";
import { showErrorToast } from "@/components/Homepage";
import { userAtom } from "@/store/atoms";
import { signUp } from "@/utils/api";
import { useAtom } from "jotai";
import { useRouter } from "next/navigation";
import { useState } from "react";
import jwt from "jsonwebtoken";


export default function SignUp() {
    const [user, setUser] = useAtom(userAtom);
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const router = useRouter();

    async function saveUser() {
        const response = await signUp({
            email: email,
            username: username,
            password: password
        });
        console.log("POST RESPONSE");
        console.log(response);
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

    return <AuthBoxInputs 
        name="Sign Up" 
        argsTupleArray=
            {
                [
                    ["Email", setEmail], 
                    ["Username", setUsername],
                    ["Password", setPassword]
                ]
            } 
        handleSubmit={saveUser}
        isSignUp={true}
    />
}