import { grey } from "@mui/material/colors";
import palette from "../styles/palette.module.scss";

/*
 * Light
 */
export const paletteLight = {
    primary: {
        main: palette.primaryWhite,
        light: palette.lightRed,
    },
    secondary: {
        main: palette.contrastBlack,
    },
    tertiary: {
        main: palette.contrastBlue,
    },
    textShadow: {
        main: palette.accentYellow,
        light: palette.secondaryRed,
        dark: palette.backgroundGrey,
    },
    background: {
        // Toggles the bg color of dropdown menu
        default: "#FFFFFF",
        paper: palette.primaryWhite,
    },
    divider: grey[200],
    text: {
        primary: palette.primaryWhite,
        secondary: palette.contrastBlack,
    },
};

/*
 * Dark
 */
export const paletteDark = {
    primary: {
        main: grey[900],
        light: grey[800],
    },
    secondary: {
        main: paletteLight.secondary.main,
    },
    tertiary: {
        main: palette.contrastBlue,
    },
    textShadow: {
        main: paletteLight.textShadow.main,
        light: paletteLight.textShadow.light,
        dark: paletteLight.textShadow.dark,
    },
    divider: grey[700],
    background: {
        default: grey[700],
        paper: grey[500],
    },
    text: {
        primary: palette.baseYellow,
        secondary: palette.contrastBlack,
    },
};
