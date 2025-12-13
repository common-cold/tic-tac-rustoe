"use client"

import { AuthBoxInputs } from "@/components/auth/AuthBox";
import { showErrorToast } from "@/components/Homepage";
import { signIn } from "@/utils/api";
import { useRouter } from "next/navigation";
import { useState } from "react";


export default function SignIn() {
    const [password, setPassword] = useState("");
    const [username, setUsername] = useState("");
    const router = useRouter();

    async function handleSignIn() {
        const response = await signIn({
            username: username,
            password: password
        });
        if (!response || response.status != 200) {
            console.log("SINGNINNNNNNN")
            showErrorToast("Unable to Signup");
        } else if (response.status === 200) {
            const data = response.data as any;
            const token = data.token;
            localStorage.setItem("token", token);
            router.push("/");
        }    
    } 

    return <>
        <button onClick={() => {
            console.log("CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCc");
            showErrorToast("Hello")}}>
            Hellllllllll
        </button>
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