import { CreateGame } from "../CreateGame";
import { LeaveRoom } from "../LeaveRoom";
import RejoinRoom from "../RejoinRoom";

export type GameModalMenuProps = {
    title: string | null
}


export default function GameModalMenu({title} : GameModalMenuProps) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center">
      <div className="flex flex-col bg-[#161b22] rounded-lg py-10 w-3/8 h-3/6 gap-30">
        <div className="text-center font-bold text-[30px]">
            {title}
        </div>
        <div className="flex flex-row justify-around items-center">
            <RejoinRoom/>
            <LeaveRoom/>
        </div>
      </div>
    </div>
  );
}
