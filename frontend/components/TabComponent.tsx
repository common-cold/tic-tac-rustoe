"use client"

type TabComponentProps = {
    tab: number, 
    setTab: (index: number) => void, 
    index: number, 
    label: string
}


export function TabComponent({tab, setTab, index, label} : TabComponentProps) {
    return <div style={{color: tab === index ? "#ff6f5c" : "white", display: "flex", flexDirection: "column", width: "100px", gap: "5px"}} 
        onClick={() => {
            setTab(index);
    }}>
        <div className="flex justify-center cursor-pointer hover:text-[#ff6f5c] transition delay-100">
            {label}
        </div>
        {
            tab === index 
            &&
            <div style={{width: "100%", height: "2px", backgroundColor: "#ff6f5c", alignItems: "center", justifyContent: "center"}}/>
        }
        
    </div>
}