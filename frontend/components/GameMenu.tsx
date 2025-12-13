import { CreateRoom } from "./CreateRoom"
import { JoinRoom } from "./JoinRoom"
import { CreateGame } from "./CreateGame"


export function GameMenu() {

    return <div className="thinBorder flex flex-col items-center w-1/4 gap-5 py-4 px-5">
        <CreateRoom/>
        <JoinRoom/>
        <CreateGame/>
    </div>
}