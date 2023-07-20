import {
    ToggleButtonGroup,
    ToggleButton,
    InputLabel,
    Select,
    MenuItem,
    SelectChangeEvent,
} from "@mui/material";
import { SPaper } from "../../Styled";
import { months } from "../../enums";
import { useEffect } from "react";

interface Props {
    month: months;
    setMonth: React.Dispatch<React.SetStateAction<months>>;
}

function Month({ month, setMonth }: Props) {
    const handleChange = (event: SelectChangeEvent) => {
        const newMonth = months[event.target.value as keyof typeof months];
        setMonth(newMonth);
    };
    const keys = Object.keys(months).filter((v) => isNaN(Number(v)));
    return (
        <SPaper>
            <InputLabel id="select-label">Месяц</InputLabel>
            <Select
                labelId="select-label"
                id="month-select"
                value={months[month].toString()}
                label="Месяц"
                onChange={handleChange}
            >
                {keys.map((m) => (
                    <MenuItem value={m}>{m}</MenuItem>
                ))}
            </Select>
        </SPaper>
    );
}

export default Month;
