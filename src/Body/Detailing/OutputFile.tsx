import { Box, Grid, Stack, Button, Typography, Tab, Tabs, Input } from "@mui/material";
import { SPaper } from "../../Styled";
import { useState } from "react";
import NewFile from "./OutputFile/NewFile";
import SelectFile from "./OutputFile/SelectFile";

interface Props {
    inputFile: string;
    outputFolder: string;
    outputFileName: string;
    setOutputFolder: React.Dispatch<React.SetStateAction<string>>;
    setOutputFilename: React.Dispatch<React.SetStateAction<string>>;
    lastFolder: string;
    setLastFolder: React.Dispatch<React.SetStateAction<string>>;
}
function OutputFile({
    inputFile,
    outputFolder,
    outputFileName,
    setOutputFilename,
    setOutputFolder,
    lastFolder,
    setLastFolder,
}: Props) {
    const [tabValue, setTabValue] = useState(0);

    const handleChange = (event: React.SyntheticEvent, newValue: number) => {
        setTabValue(newValue);
    };

    return (
        <SPaper>
            <Box
                sx={{
                    p: 2,
                    pl: 0,
                }}
            >
                <Grid container spacing={2}>
                    <Grid item xs={5} sx={{ borderRight: 1, borderColor: "divider" }}>
                        <Tabs
                            orientation="vertical"
                            value={tabValue}
                            onChange={handleChange}
                            aria-label="Файл"
                        >
                            <Tab label="Новый файл" />
                            <Tab label="Записать в существующий файл" />
                        </Tabs>
                    </Grid>
                    <Grid item xs={7}>
                        {tabValue === 0 && (
                            <NewFile
                                inputFile={inputFile}
                                outputFileName={outputFileName}
                                outputFolder={outputFolder}
                                setOutputFilename={setOutputFilename}
                                setOutputFolder={setOutputFolder}
                                lastFolder={lastFolder}
                                setLastFolder={setLastFolder}
                            />
                        )}
                        {tabValue === 1 && (
                            <SelectFile
                                inputFile={inputFile}
                                outputFileName={outputFileName}
                                outputFolder={outputFolder}
                                setOutputFilename={setOutputFilename}
                                setOutputFolder={setOutputFolder}
                                lastFolder={lastFolder}
                                setLastFolder={setLastFolder}
                            />
                        )}
                    </Grid>
                </Grid>
            </Box>
        </SPaper>
    );
}

export default OutputFile;
