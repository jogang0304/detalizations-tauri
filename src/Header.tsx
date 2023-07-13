import SettingsIcon from "@mui/icons-material/Settings";
import { AppBar, Grid, Stack, Tab, Tabs, Typography } from "@mui/material";
import React, { useEffect } from "react";
import { getVersion } from "@tauri-apps/api/app";

const appVersion = await getVersion();

interface Props {
    tab: number;
    setTab: React.Dispatch<React.SetStateAction<number>>;
}
function Header({ tab, setTab }: Props): React.JSX.Element {
    const handleChange = (event: React.SyntheticEvent, newValue: number) => {
        setTab(newValue);
    };
    return (
        <AppBar
            position="static"
            color="secondary"
            sx={{
                pr: 2,
            }}
        >
            <Stack direction="row" alignItems="center" justifyContent="space-between">
                <Tabs value={tab} onChange={handleChange} aria-label="Вкладки">
                    <Tab label="Детализация" />
                    <Tab label="Пересчёт себестоимости" />
                </Tabs>
                <Typography>Версия {appVersion}</Typography>
                <SettingsIcon />
            </Stack>
        </AppBar>
    );
}

export default Header;
