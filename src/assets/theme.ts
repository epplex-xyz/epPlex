import {createTheme, responsiveFontSizes} from "@mui/material/styles";
import palette from "../styles/palette.module.scss";
import style from "../styles/style.module.scss";
import {paletteDark, paletteLight} from "./palette";
import {PaletteMode} from "@mui/material";
import {BasteleurBold} from "./fonts";

declare module '@mui/material/styles' {
    interface Theme {
        customVariables: {
            mainBackgroundColor: string;
        };
    }
    // fix the type error when calling `createTheme()` with a custom theme option
    interface ThemeOptions {
        customVariables: {
            mainBackgroundColor: string;
        };
    }

    interface Palette {
        textShadow: Palette['primary'];
    }
    interface PaletteOptions {
        textShadow: PaletteOptions['primary'];
    }
    interface Palette {
        tertiary: Palette['primary'];
    }
    interface PaletteOptions {
        tertiary: PaletteOptions['primary'];
    }

    // palette.primary.darker
    // interface PaletteColor {
    //     darker?: string;
    // }
    // interface SimplePaletteColorOptions {
    //     darker?: string;
    // }
}


const theme = (colorMode: PaletteMode) => responsiveFontSizes(createTheme({
    customVariables: {
        mainBackgroundColor:
                (colorMode === 'light' ? "url(images/bgGradient.png)" : "url(images/darkBackground.svg)")
    },
    breakpoints: {
        values: {
            xs: 0,
            sm: 600,
            md: 900,
            lg: 1200,
            xl: 1536,
        },
    },
    palette: {
        mode: colorMode,
        ...(colorMode === 'light' ? paletteLight : paletteDark),
    },
    typography: {
    // If less than or equal to 600 then do this
        h1: {
            '@media (max-width:600px)': {
                fontSize: '3rem',
            },
        },
        allVariants: {
            fontFamily: [
                'SatoshiMedium',
                'Bubblegum Sans',
                'Roboto',
                '-apple-system',
                'BlinkMacSystemFont',
                '"Segoe UI"',
                '"Helvetica Neue"',
                'Arial',
                'sans-serif',
                '"Apple Color Emoji"',
                '"Segoe UI Emoji"',
                '"Segoe UI Symbol"',
            ].join(',')
        },
    },
    components: {
        MuiButton: {
            defaultProps: {
                disableRipple: true,
                // color: "secondary",
            },
            variants: [
                {
                    props: { variant: 'text' },
                    style: {
                        '&:hover': {
                            // backgroundColor: paletteLight.secondary.main,
                            backgroundColor: "rgba(0, 0, 0, 0.04)"
                        },
                    },
                },
                {
                    props: { variant: 'contained' },
                    style: {
                        // backgroundColor: palette.contrastBlack,
                        '&:hover': {
                            backgroundColor: `${palette.contrastBlack}80`
                        },
                    },
                }
            ],
            styleOverrides: {
                root: {
                    borderRadius: style.borderRadiusMd,
                    minWidth: 5,
                    textTransform: "none",
                    fontFamily: BasteleurBold.style.fontFamily,
                    color: paletteLight.secondary.main,
                },
            },
        },
        // Fix disappearing of scrollbar, in the navbar menu
        // https://stackoverflow.com/questions/69065717/material-ui-menu-component-locks-body-scrollbar/71671897#71671897
        MuiMenu: {
            defaultProps: {
                disableScrollLock: true,
            },
        },
        // FIx disappearing of scrollbar, when selecting wallet
        MuiDialog: {
            defaultProps: {
                disableScrollLock: true,
            },
        },
        MuiLink: {
            defaultProps: {
                underline: 'none',
                variant: 'button'
            },
            styleOverrides: {
                root: {
                    color: palette.black,
                    '&:hover': {}, // no hover color
                },
            },
        },
        MuiCard: {
            styleOverrides: {
                root: {
                    borderRadius: style.borderRadiusMd
                }
            },
        },

    },
}));

export default theme;


