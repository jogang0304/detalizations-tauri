import { Button } from "@mui/material";
import { SPaper } from "../../Styled";
import { marketplaces } from "../../enums";
import { invoke } from "@tauri-apps/api/tauri";

interface Props {
    inputFile: string;
    outputFolder: string;
    outputFilename: string;
    marketplace: marketplaces;
}
function Confirm({ inputFile, outputFilename, outputFolder, marketplace }: Props) {
    const handleConfirm = async () => {
        let outputFile = outputFolder + "/" + outputFilename;
        if (outputFilename == "") {
            outputFile += "БУХ " + inputFile.split(/(\\|\/)/g).pop();
        }

        const result = await invoke<[boolean, string]>("handle_confirm", {
            inputFile: inputFile,
            outputFile: outputFile,
            marketplace: marketplaces[marketplace],
        });
        alert(result[1]);
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
