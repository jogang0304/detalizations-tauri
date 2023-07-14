import { Button } from "@mui/material";
import { SPaper } from "../../Styled";
import { invoke } from "@tauri-apps/api/tauri";

interface Props {
    targetDir: string;
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}
function Confirm({ targetDir, setMessage }: Props) {
    const handleConfirm = async () => {
        setMessage("Ждите");
        if (targetDir == "") {
            targetDir = ".";
        }

        const result = await invoke<[boolean, string]>("handle_price_confirm", {
            targetDir: targetDir,
        });
        if (result[0] == true) {
            setMessage(result[1]);
        } else {
            setMessage("Ошибка. " + result[1]);
        }
    };

    return (
        <SPaper>
            <Button onClick={handleConfirm} variant="contained">
                Посчитать
            </Button>
        </SPaper>
    );
}

export default Confirm;
