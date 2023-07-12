import SettingsIcon from "@mui/icons-material/Settings";
import { AppBar, Grid, Stack, Tab, Tabs } from "@mui/material";
import React from "react";

interface Props {
    tab: number;
    setTab: React.Dispatch<React.SetStateAction<number>>;
}
function Header({ tab, setTab }: Props): React.JSX.Element {
    const handleChange = (event: React.SyntheticEvent, newValue: number) => {
        setTab(newValue);
    };
    return (
        <AppBar position="static" color="secondary" sx={{
            pr: 2
        }}>
            <Stack direction="row" alignItems="center" justifyContent="space-between">
                <Tabs value={tab} onChange={handleChange} aria-label="Вкладки">
                    <Tab label="Детализация" />
                    <Tab label="Пересчёт себестоимости" />
                </Tabs>
                <SettingsIcon />
            </Stack>
        </AppBar>
    );
}

export default Header;
