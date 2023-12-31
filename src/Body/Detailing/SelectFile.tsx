import { Box, Button, Grid, Paper, Stack, Typography } from "@mui/material";
import { useState } from "react";
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { SPaper } from "../../Styled";

interface Props {
    filePath: string;
    setFilePath: React.Dispatch<React.SetStateAction<string>>;
    lastFolder: string;
    setLastFolder: React.Dispatch<React.SetStateAction<string>>;
}

function SelectFile({ filePath, setFilePath, lastFolder, setLastFolder }: Props) {
    const handleSelect = async () => {
        const selected = await open({
            directory: false,
            filters: [
                {
                    name: "Excel",
                    extensions: ["xlsx", "xls", "xlsm"],
                },
            ],
            multiple: false,
            defaultPath: lastFolder,
        });
        if (selected !== null && !Array.isArray(selected)) {
            setFilePath(selected);
            let fileFolder = selected.split(/(\\|\/)/g).slice(0, -1).join("/");
            setLastFolder(fileFolder);
        }
    };

    return (
        <SPaper>
            <Box
                sx={{
                    p: 2,
                }}
            >
                <Grid container>
                    <Grid xs={12}>
                        <Stack alignItems="center" spacing={2} justifyContent="center">
                            <Button onClick={handleSelect} variant="contained">
                                Загрузить
                            </Button>
                            <Typography>{filePath ? filePath : "Файл детализации"}</Typography>
                        </Stack>
                    </Grid>
                </Grid>
            </Box>
        </SPaper>
    );
}

export default SelectFile;
