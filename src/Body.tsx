import { Box, Grid } from "@mui/material";

import { appConfigDir, appLocalDataDir, dataDir } from "@tauri-apps/api/path";
import Detailing from "./Body/Detailing";

const dDir = await dataDir();
const configDir = await appConfigDir();
const localDir = await appLocalDataDir();

interface Props {
    tab: number;
}
function Body({ tab }: Props) {
    return (
        <Box
            sx={{
                p: 2,
            }}
        >
            {tab === 0 && <Detailing />}
        </Box>
    );
}

export default Body;
