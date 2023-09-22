import { grey } from "@mui/material/colors";
import palette from "../styles/palette.module.scss";

/*
 * Light
 */

export const paletteLight = {
    primary: {
        main: palette.contrastBlack,
    },
    secondary: {
        main: palette.primaryWhite,
    },
    textShadow: {
        main: palette.accentYellow,
        light: palette.secondaryRed,
        dark: palette.backgroundGrey,
    },
    background: {
        // Toggles the bg color of dropdown menu
        default: palette.primaryWhite,
        paper: palette.primaryWhite,
    },
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
    },
    secondary: {
        main: paletteLight.secondary.main,
    },
    textShadow: {
        main: paletteLight.textShadow.main,
        light: paletteLight.textShadow.light,
        dark: paletteLight.textShadow.dark,
    },
    background: {
        default: grey[700],
        paper: grey[500],
    },
    text: {
        primary: palette.baseYellow,
        secondary: palette.contrastBlack,
    },
};
