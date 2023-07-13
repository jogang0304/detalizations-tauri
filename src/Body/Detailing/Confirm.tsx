import { Button } from "@mui/material";
import { SPaper } from "../../Styled";
import { marketplaces } from "../../enums";
import { invoke } from "@tauri-apps/api/tauri";

interface Props {
    inputFile: string;
    outputFolder: string;
    outputFilename: string;
    marketplace: marketplaces;
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}
function Confirm({ inputFile, outputFilename, outputFolder, marketplace, setMessage }: Props) {
    const handleConfirm = async () => {
        if (outputFilename == "") {
            outputFilename = "БУХ " + inputFile.split(/(\\|\/)/g).pop();
        }

        if (!inputFile) {
            setMessage("Нет входного файла");
        } else if (!(outputFilename && outputFolder)) {
            setMessage("Выберите папку");
        } else {
            let outputFile = outputFolder + "/" + outputFilename;

            const result = await invoke<[boolean, string]>("handle_confirm", {
                inputFile: inputFile,
                outputFile: outputFile,
                marketplace: marketplaces[marketplace],
            });
            if (result[0] == true) {
                setMessage("Записан файл " + result[1]);
            } else {
                setMessage("Ошибка. " + result[1]);
            }
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
