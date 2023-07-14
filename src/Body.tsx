import { Box, Grid } from "@mui/material";

import Detailing from "./Body/Detailing";
import Price from "./Body/Price";

interface Props {
    tab: number;
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}
function Body({ tab, setMessage }: Props) {
    return (
        <Box
            sx={{
                p: 2,
            }}
        >
            {tab === 0 && <Detailing setMessage={setMessage} />}
            {tab === 1 && <Price setMessage={setMessage} />}
        </Box>
    );
}

export default Body;
