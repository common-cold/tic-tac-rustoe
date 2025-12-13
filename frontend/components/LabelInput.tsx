export type LabelInputProps = {
    label: string,
    setter:  React.Dispatch<React.SetStateAction<string | null>>
}

export type LabelInputNumberProps = {
    label: string,
    setter:  React.Dispatch<React.SetStateAction<Number>>
}

export function LabelInput({label, setter} : LabelInputProps) {    
    return <div className="w-3/4 flex flex-col gap-2">
        <div className="font-medium">
            {label}
        </div>
        <input className="inputStyle" onChange={(e) => setter(e.target.value)}/>
    </div>
}

export function LabelInputNumber({label, setter} : LabelInputNumberProps) {    
    return <div className="w-3/4 flex flex-col gap-2">
        <div className="font-medium">
            {label}
        </div>
        <input type="number" className="inputStyle" onChange={(e) => setter(Number.parseInt(e.target.value))}/>
    </div>
}