import {createTheme, responsiveFontSizes} from "@mui/material/styles";
import style from "../styles/style.module.scss";
import {paletteDark, paletteLight} from "./palette";
import {PaletteMode} from "@mui/material";

const theme = (colorMode: PaletteMode) => responsiveFontSizes(
    createTheme({
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

                },
                variants: [
                    {
                        props: { variant: 'contained' },
                        style: {
                            backgroundColor: paletteLight.secondary.main,
                            '&:hover': {
                                backgroundColor: `${paletteLight.secondary.main}` + "80"
                            },
                        },
                    },
                    {
                        props: { variant: 'outlined' },
                        style: {
                            borderColor: paletteLight.secondary.main,
                            '&:hover': {
                                backgroundColor: `${paletteLight.primary.main}` + "80"
                            },
                        },
                    }
                ],
                styleOverrides: {
                    root: {
                        borderRadius: style.borderRadiusMd,
                        minWidth: 5,
                        textTransform: "none",
                        color: paletteLight.primary.main,
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
                        color: "text.secondary",
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
    }), { factor: 1.2 });

export default theme;


