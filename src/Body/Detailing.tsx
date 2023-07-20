import { Grid, Stack } from "@mui/material";
import SelectFile from "./Detailing/SelectFile";
import { useState } from "react";
import OutputFile from "./Detailing/OutputFile";
import Marketplace from "./Detailing/Marketplace";
import { marketplaces, months } from "../enums";
import Confirm from "./Detailing/Confirm";
import Month from "./Detailing/Month";
import { documentDir } from "@tauri-apps/api/path";

const dd = await documentDir();
interface Props {
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}
function Detailing({ setMessage }: Props) {
    const [inputFile, setInputFile] = useState("");
    const [outputFolder, setOutputFolder] = useState("");
    const [outputFilename, setOutputFilename] = useState("");
    const [marketplace, setMarketplace] = useState(marketplaces.Ozon);
    const [month, setMonth] = useState(months.любой);

    const [lastFolder, setLastFolder] = useState(dd);

    return (
        <Stack spacing={2}>
            <SelectFile
                filePath={inputFile}
                setFilePath={setInputFile}
                lastFolder={lastFolder}
                setLastFolder={setLastFolder}
            />
            <OutputFile
                inputFile={inputFile}
                outputFileName={outputFilename}
                outputFolder={outputFolder}
                setOutputFilename={setOutputFilename}
                setOutputFolder={setOutputFolder}
                lastFolder={lastFolder}
                setLastFolder={setLastFolder}
            />
            <Marketplace marketplace={marketplace} setMarketplace={setMarketplace} />
            <Month month={month} setMonth={setMonth} />
            <Confirm
                inputFile={inputFile}
                outputFilename={outputFilename}
                outputFolder={outputFolder}
                marketplace={marketplace}
                month={month}
                setMessage={setMessage}
            />
        </Stack>
    );
}

export default Detailing;
