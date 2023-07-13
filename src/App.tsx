import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { Stack } from "@mui/material";
import Header from "./Header";
import Body from "./Body";
import Info from "./Info";

function App() {
    const [tab, setTab] = useState(0);
    const [message, setMessage] = useState("");

    return (
        <Stack>
            <Header tab={tab} setTab={setTab} />
            <Body tab={tab} setMessage={setMessage} />
            <Info message={message} setMessage={setMessage} />
        </Stack>
    );
}

export default App;
