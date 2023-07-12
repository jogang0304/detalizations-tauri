import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./main.css";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import blue from "@mui/material/colors/blue";



const theme = createTheme({
    palette: {
        secondary: {
            main: "#FFDDA2",
        },
        primary: {
            main: "#000E56",
        },
    },
    components: {
        MuiButton: {
            defaultProps: {
                color: "secondary",
                
            },
        },
        MuiPaper: {
            defaultProps: {
                
            },
        },
    },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <ThemeProvider theme={theme}>
            <App />
        </ThemeProvider>
    </React.StrictMode>,
);
