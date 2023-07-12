import { Stack, Button, Input, Typography } from "@mui/material";
import { documentDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";

interface Props {
    inputFile: string;
    outputFolder: string;
    outputFileName: string;
    setOutputFolder: React.Dispatch<React.SetStateAction<string>>;
    setOutputFilename: React.Dispatch<React.SetStateAction<string>>;
}

function SelectFile({
    inputFile,
    outputFolder,
    outputFileName,
    setOutputFilename,
    setOutputFolder,
}: Props) {
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
            defaultPath: await documentDir(),
        });
        if (selected !== null && !Array.isArray(selected)) {
            const splittedFile = selected.split(/(\\|\/)/g);
            setOutputFilename(splittedFile[splittedFile.length - 1]);
            let folderPath = "";
            splittedFile.forEach((x, i) => {
                if (i < splittedFile.length - 2) {
                    folderPath += x;
                }
            });
            setOutputFolder(folderPath);
        }
    };

    return (
        <Stack alignItems="center" spacing={2} justifyContent="center">
            <Button onClick={handleSelect} variant="contained">
                Выбрать файл
            </Button>
            <Typography>{outputFolder + "/" + outputFileName}</Typography>
        </Stack>
    );
}

export default SelectFile;
