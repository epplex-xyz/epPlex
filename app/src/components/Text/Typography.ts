import { useMemo } from "react";
import { useTheme } from "@mui/material/styles";
import { SatoshiBold, AlegreyaBold} from "@styles/fonts";

const mainFont = SatoshiBold.style.fontFamily;
const secondaryFont = AlegreyaBold.style.fontFamily;

export type FontVariant = {
    primary: string;
    secondary?: string;
    bold?: string;

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

const defaultFont: TypographyVariant = {
    font: {
        primary: mainFont,
        secondary: secondaryFont,
    }
};


export const typography: TypographyVariants = {
    h1: defaultFont,
    h2: defaultFont,
    h3: defaultFont,
    h4: defaultFont,
    h5: defaultFont,
    h6: defaultFont,
    subtitle1: defaultFont,
    body1: defaultFont,
    body2: defaultFont,
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
            case "secondary":
                return typography[props.typographyVariant].font?.secondary;
            case "primary":
            default:
                return typography[props.typographyVariant].font?.primary;
        }
    }, [props.fontVariant, props.typographyVariant]);
    return fontFamily;
};