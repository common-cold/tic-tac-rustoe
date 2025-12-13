import { Signout } from "./auth/Signout";


export function Appbar() {
    
    
    return <div className="flex justify-between items-center">
        <div className="primaryTextColor font-satoshi font-extrabold text-5xl">
            TicTacToe
        </div>
        <Signout/>
    </div>
}