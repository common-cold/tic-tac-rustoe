import { showErrorToast, showSuccessToast } from "@/components/Homepage";
import { Room } from "@/types/db";
import { LogArgs, WebSocketResponseWrapper } from "@/types/ws";
import { atom } from "jotai";

export const wsAtom = atom<WebSocket | null>(null);

export const connectSocketAtom = atom(null, (get, set) => {
    if (get(wsAtom) != null) {
        return;
    }

    const token = localStorage.getItem("token");
    let websocket = new WebSocket(`ws://localhost:8081/ws?token=${token}`);

    websocket.onopen = () => console.log("Ws Connected");

    websocket.onmessage = (data) => {
        let responseWrapper: WebSocketResponseWrapper = JSON.parse(data.data) ;
        let response = responseWrapper.data;
        if (response.type == 'Log') {
            let log: LogArgs = response.payload;
            console.log(log);
            if (log.isError) {
                showErrorToast(log.message)
            } else {
                showSuccessToast(log.message)
            }
        }
    };

    websocket.onclose = () => {
        set(wsAtom, null);
    }

    set(wsAtom, websocket);
});

export const roomAtom = atom<Room | null>(null);