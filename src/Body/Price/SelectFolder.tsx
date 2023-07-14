import { Stack, Button, Typography, Box } from "@mui/material";
import { documentDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import { SPaper } from "../../Styled";

interface Props {
    targetDir: string;
    setTargetDir: React.Dispatch<React.SetStateAction<string>>;
}
function SelectFolder({ targetDir, setTargetDir }: Props) {
    const handleSelect = async () => {
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await documentDir(),
        });
        if (selected !== null && !Array.isArray(selected)) {
            setTargetDir(selected);
        }
    };

    return (
        <SPaper>
            <Box
                sx={{
                    p: 2,
                    pl: 0,
                }}
            >
                <Stack alignItems="center" spacing={2} justifyContent="center">
                    <Button onClick={handleSelect} variant="contained">
                        Выбрать папку
                    </Button>
                    <Typography>{targetDir}</Typography>
                </Stack>
            </Box>
        </SPaper>
    );
}

export default SelectFolder;
