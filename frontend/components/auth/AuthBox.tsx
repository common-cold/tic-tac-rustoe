
export type AuthBoxInputProps = {
    name: string,
    argsTupleArray: [string, (val: string) => void][]
    handleSubmit: () => Promise<void>
    isSignUp: boolean
}

export function AuthBoxInputs({name, argsTupleArray, handleSubmit, isSignUp}: AuthBoxInputProps) {
    return <div className={`flex flex-col gap-5 secondaryBg ${isSignUp ? "h-[400px]" : "h-[300px]"} w-[400px] rounded-[10px] py-3 px-6`}>
        <div className="flex text-white text-2xl font-bold items-center justify-center">
            {name}
        </div>
        {
            argsTupleArray.map(([label, setter], i) => {
                return <div key={i} className="flex flex-col text-white gap-1">
                    <div className="flex text-white text-lg font-bold justify-start">
                        {label}
                    </div>
                    <div>
                        <input 
                            className="inputStyle w-full" 
                            onChange={(e) => setter(e.target.value)}
                            type =  {label == "Password" ? "password" : "text"}
                        />
                    </div>
                </div>
            })
        }
        
        <div onClick={() => {
            async function callHandleSubmit() {
                await handleSubmit();
            }

            callHandleSubmit();
        }} 
            className="flex items-center justify-center h-10 w-full mt-2 primaryButton text-white font-bold rounded-[10px] cursor-pointer">
                Submit
        </div>
    </div>
}