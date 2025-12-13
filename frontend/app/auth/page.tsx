"use client"

import { AuthHeaderComponent } from "@/components/auth/AuthHeaderComponent";
import SignIn from "@/components/auth/SignIn";
import SignUp from "@/components/auth/SignUp";

import { TabComponent } from "@/components/TabComponent";
import { useState } from "react";


export default function AuthPage() {
    const [tab, setTab] = useState(0);

    return <div className="flex-1 py-5 px-20 h-screen">
        <div className="flex flex-col justify-center items-center gap-10">
            <AuthHeaderComponent/>
            <div className="flex justify-start gap-6">
                <TabComponent tab={tab} setTab={setTab} index={0} label="SignUp" />
                <TabComponent tab={tab} setTab={setTab} index={1} label="SignIn" />
            </div>
            {
                tab === 0
                ?
                <SignUp/>
                :
                <SignIn/>
            }
        </div>
    </div>
}