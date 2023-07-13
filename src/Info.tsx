import { Box, Slide, SlideProps, Snackbar } from "@mui/material";
import { useEffect } from "react";

interface Props {
    message: string;
    setMessage: React.Dispatch<React.SetStateAction<string>>;
}

function SlideTransition(props: SlideProps) {
    return <Slide {...props} direction="up" />;
}

function Info({ message, setMessage }: Props) {
    const opened = message !== "";
    console.log(message);

    return (
        <Snackbar
            autoHideDuration={6000}
            anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
            open={opened}
            onClose={() => setMessage("")}
            TransitionComponent={SlideTransition}
            message={message}
        />
    );
}

export default Info;
