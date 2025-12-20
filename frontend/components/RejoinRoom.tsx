import { gameAtom, refreshGamePageAtom, roomAtom, showGameMenuModalAtom } from "@/store/atoms";
import { useAtom } from "jotai";


export default function RejoinRoom() {
    const [showGameMenuModal, setShowGameMenuModal] = useAtom(showGameMenuModalAtom);
    const [game, setGame] = useAtom(gameAtom);
    const [room, setRoom] = useAtom(roomAtom);
    const [refreshGamePage, setRefreshGamePage] = useAtom(refreshGamePageAtom);

    function handleClick() {
        setShowGameMenuModal(false)
        setGame(null);
        setRoom(null);
        setRefreshGamePage(prev => !prev);
    }

    return <div>
        <button className="rounded-[7px] w-[100px] h-[50px] primaryButton font-bold"
            onClick={handleClick}>
            Rejoin Room
        </button>
    </div>
}