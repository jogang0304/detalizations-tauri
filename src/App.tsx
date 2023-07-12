import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { Stack } from "@mui/material";
import Header from "./Header";
import Body from "./Body";



function App() {
    const [tab, setTab] = useState(0);
    

    return (
        <Stack>
            <Header tab={tab} setTab={setTab}/>
            <Body tab={tab}/>
        </Stack>
    );
}

export default App;
