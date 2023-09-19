import { useMemo } from "react";
import { useTheme } from "@mui/material/styles";
import { BasteleurBold, SatoshiBold, SatoshiMedium } from "../../assets/fonts";

const mainFont = BasteleurBold.style.fontFamily;
const secondaryFont = SatoshiMedium.style.fontFamily;
const secondaryBoldFont = SatoshiBold.style.fontFamily;

export type FontVariant = {
    normal: string;
    bold?: string;
    second?: string;
};

export type FontVariantKey = keyof FontVariant;

export type TypographyVariant = {
    font?: FontVariant;
    color?: string;
    textShadow?: string;
};

export type TypographyVariants = {
    h1: TypographyVariant;
    h2: TypographyVariant;
    h3: TypographyVariant;
    h4: TypographyVariant;
    h5: TypographyVariant;
    h6: TypographyVariant;
    subtitle1: TypographyVariant;
    body1: TypographyVariant;
    body2: TypographyVariant;
};

export type TypographyVariantsKey = keyof TypographyVariants;

export const typography: TypographyVariants = {
    h1: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    h2: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    h3: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    h4: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    h5: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    h6: {
        font: {
            normal: secondaryBoldFont,
        },
    },
    subtitle1: {
        font: {
            normal: secondaryFont,
            bold: secondaryBoldFont,
            second: mainFont,
        },
    },
    body1: {
        font: {
            normal: secondaryFont,
            bold: secondaryBoldFont,

        },
    },
    body2: {
        font: {
            normal: secondaryFont,
            bold: secondaryBoldFont,
        },
    },
};

export enum TextBackground {
    red, // primary.main
    blue, // shadow.dark
    yellow, // shadow.main
    none,
}

export const useContextualTextShadowColor = (background?: TextBackground): TypographyVariant["textShadow"] => {
    const theme = useTheme();
    switch (background) {
        case TextBackground.blue:
            return theme.palette.textShadow!.dark;
        case TextBackground.yellow:
            return theme.palette.textShadow!.main;
        case TextBackground.red:
            return theme.palette.textShadow!.light;
        case TextBackground.none:
            return "transparent";
        default:
            return undefined;
    }
};

type UseTextShadowColorProps = {
    variant: TypographyVariantsKey;
    background?: TextBackground;
};

export const useTextShadowColor = (props: UseTextShadowColorProps): TypographyVariant["textShadow"] => {
    const textShadowColor = useContextualTextShadowColor(props.background);
    switch (props.variant) {
        case "h1":
            return `6px 6px 0px ${textShadowColor}`;
        case "h2":
            return `4px 4px 0px ${textShadowColor}`;
        case "h3":
            return `5px 5px 0px ${textShadowColor}`;
        case "h4":
            return `4px 4px 0px ${textShadowColor}`;
        case "h5":
            return `3px 3px 0px ${textShadowColor}`;
        case "h6":
            return `2px 2px 0px ${textShadowColor}`;
        case "subtitle1":
        case "body1":
        case "body2":
        default:
            return undefined;
    }
};

type UseFontFamilyProps = {
    typographyVariant: TypographyVariantsKey;
    fontVariant: FontVariantKey;
};

export const useFontFamily = (props: UseFontFamilyProps) => {
    const fontFamily = useMemo(() => {
        switch (props.fontVariant) {
            case "bold":
                return typography[props.typographyVariant].font?.bold;
            case "second":
                return typography[props.typographyVariant].font?.second;
            case "normal":
            default:
                return typography[props.typographyVariant].font?.normal;
        }
    }, [props.fontVariant, props.typographyVariant]);
    return fontFamily;
};