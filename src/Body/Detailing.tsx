import { Grid, Stack } from "@mui/material";
import SelectFile from "./Detailing/SelectFile";
import { useState } from "react";

function Detailing() {
    const [filePath, setFilePath] = useState("Файл детализации");

    return (
        <Stack spacing={2}>
            <SelectFile filePath={filePath} setFilePath={setFilePath} />
            {/* <OutputFile />
            <Marketplace />
            <Confirm /> */}
        </Stack>
    );
}

export default Detailing;
