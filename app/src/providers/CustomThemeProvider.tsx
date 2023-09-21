import * as React from 'react';
import {ThemeProvider} from '@mui/material/styles';
import {PaletteMode} from "@mui/material";
import useMediaQuery from '@mui/material/useMediaQuery';
import theme from "@styles/theme";
import {CacheProvider} from '@emotion/react';
import CssBaseline from "@mui/material/CssBaseline";

export const ColorModeContext = React.createContext({ toggleColorMode: () => {} });

export const CustomThemeProvider = ({children, cache}) => {
    const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');
    const initial = prefersDarkMode ? 'dark' : 'light';
    const [mode, setMode] = React.useState<PaletteMode>(initial);

    const colorMode = React.useMemo(
        () => ({
            toggleColorMode: () => {
                setMode((prevMode) => (prevMode === 'light' ? 'dark' : 'light'));
            },
        }),
        [],
    );

    const colorModeTheme = React.useMemo(
        () => theme(mode),
        [mode],
    );

    return (
        <CacheProvider value={cache}>
            <ColorModeContext.Provider value={colorMode}>
                <ThemeProvider theme={colorModeTheme}>
                    <CssBaseline/>
                    {children}
                </ThemeProvider>
            </ColorModeContext.Provider>
        </CacheProvider>
    );
};