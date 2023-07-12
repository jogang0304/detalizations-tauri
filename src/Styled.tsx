import Paper from "@mui/material/Paper";
import { styled } from "@mui/material/styles";

const SPaper = styled(Paper)(({ theme }) => ({
    backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#FAFAFA",
    ...theme.typography.body2,
    padding: theme.spacing(1),
    textAlign: "center",
    color: theme.palette.text.secondary,
    flexGrow: 1,
}));

export { SPaper };
