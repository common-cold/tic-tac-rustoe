import { CreateRoom } from "./CreateRoom"
import { JoinRoom } from "./JoinRoom"


export function GameMenu() {

    return <div className="flex flex-row justify-between items-center gap-5 py-4 px-5 w-full">
        <CreateRoom/>
        <JoinRoom/>
    </div>
}