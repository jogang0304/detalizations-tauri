import { Stack, Button, Input, Typography, TextField } from "@mui/material";
import { documentDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";

interface Props {
    inputFile: string;
    outputFolder: string;
    outputFileName: string;
    setOutputFolder: React.Dispatch<React.SetStateAction<string>>;
    setOutputFilename: React.Dispatch<React.SetStateAction<string>>;
    lastFolder: string;
    setLastFolder: React.Dispatch<React.SetStateAction<string>>;
}

function NewFile({
    inputFile,
    outputFolder,
    outputFileName,
    setOutputFilename,
    setOutputFolder,
    lastFolder,
    setLastFolder,
}: Props) {
    const handleSelect = async () => {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: lastFolder,
        });
        if (selected !== null && !Array.isArray(selected)) {
            setOutputFolder(selected);
            setLastFolder(selected);
        }
    };
    const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setOutputFilename(e.target.value);
    };

    return (
        <Stack alignItems="center" spacing={2} justifyContent="center">
            <Button onClick={handleSelect} variant="contained">
                Выбрать папку
            </Button>
            <Typography>{outputFolder}</Typography>
            <TextField
                placeholder={"БУХ " + inputFile.split(/(\\|\/)/g).pop()}
                onChange={handleNameChange}
                value={outputFileName}
                fullWidth
                label="Название файла"
                variant="standard"
            />
        </Stack>
    );
}

export default NewFile;
