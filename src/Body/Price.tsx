import { Stack } from "@mui/material";
import { useState } from "react";
import SelectFolder from "./Price/SelectFolder";
import Confirm from "./Price/Confirm";

interface Props {
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}
function Price({ setMessage }: Props) {
    const [targetDir, setTargetDir] = useState(".");
    return (
        <Stack spacing={2}>
            <SelectFolder targetDir={targetDir} setTargetDir={setTargetDir} />
            <Confirm targetDir={targetDir} setMessage={setMessage} />
        </Stack>
    );
}

export default Price;
