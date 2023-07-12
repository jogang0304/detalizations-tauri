import { Grid, Stack } from "@mui/material";
import SelectFile from "./Detailing/SelectFile";
import { useState } from "react";
import OutputFile from "./Detailing/OutputFile";
import Marketplace from "./Detailing/Marketplace";
import { marketplaces } from "../enums";
import Confirm from "./Detailing/Confirm";

function Detailing() {
    const [inputFile, setInputFile] = useState("Файл детализации");
    const [outputFolder, setOutputFolder] = useState("");
    const [outputFilename, setOutputFilename] = useState("");
    const [marketplace, setMarketplace] = useState(marketplaces.Ozon);

    return (
        <Stack spacing={2}>
            <SelectFile filePath={inputFile} setFilePath={setInputFile} />
            <OutputFile
                inputFile={inputFile}
                outputFileName={outputFilename}
                outputFolder={outputFolder}
                setOutputFilename={setOutputFilename}
                setOutputFolder={setOutputFolder}
            />
            <Marketplace marketplace={marketplace} setMarketplace={setMarketplace} />
            <Confirm
                inputFile={inputFile}
                outputFilename={outputFilename}
                outputFolder={outputFolder}
                marketplace={marketplace}
            />
        </Stack>
    );
}

export default Detailing;
