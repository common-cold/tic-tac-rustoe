import { CreateGame, CreateRoom, GetGame, JoinRoom, SignIn, SignUp } from "@/types/route";
import axios from "axios";




export async function signUp(body: SignUp) {
    try {
        const response = await axios.post("http://localhost:8080/signup", body, {
            validateStatus: () => true
        });
        return response;
    } catch (e) {
        return null
    } 
}

export async function signIn(body: SignIn) {
    try {
        const response = await axios.post("http://localhost:8080/signin", body, {
            validateStatus: () => true
        });
        return response;
    } catch (e) {
        return null
    } 
}


export async function createRoom(body: CreateRoom) {
    try {
        const token = localStorage.getItem("token");
        const response = await axios.post("http://localhost:8080/room", body, {
            validateStatus: () => true,
            headers: {
                Authorization: token
            }
        });
        return response;
    } catch (e) {
        return null
    }    
}

export async function joinRoom(body: JoinRoom) {
    try {
        const token = localStorage.getItem("token");
        const response = await axios.post("http://localhost:8080/room/join", body, {
            validateStatus: () => true,
            headers: {
                Authorization:token
            }
        });
        console.log(response);
        return response;
    } catch (e) {
        return null
    }    
}

export async function leaveRoom(body: JoinRoom) {
    try {
        const token = localStorage.getItem("token");
        const response = await axios.post("http://localhost:8080/room/leave", body, {
            validateStatus: () => true,
            headers: {
                Authorization:token
            }
        });
        return response;
    } catch (e) {
        return null
    }    
}

export async function getRoom(roomId: string) {
    try {
        const token = localStorage.getItem("token");
        const response = await axios.get(`http://localhost:8080/room/${roomId}`, {
            validateStatus: () => true,
            headers: {
                Authorization:token
            }
        });
        return response;
    } catch (e) {
        return null
    }    
}

export async function createGame(body: CreateGame) {
    console.log(body);
    try {
        const token = localStorage.getItem("token");
        const response = await axios.post("http://localhost:8080/game", body, {
            validateStatus: () => true,
            headers: {
                Authorization:token
            }
        });
        return response;
    } catch (e) {
        return null
    }  
}

export async function getGame(body: GetGame) {
    try {
        const token = localStorage.getItem("token");
        const response = await axios.post("http://localhost:8080/game/fetch", body, {
            validateStatus: () => true
        });
        return response;
    } catch (e) {
        return null
    } 
}