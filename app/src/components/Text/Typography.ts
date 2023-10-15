import { useMemo } from "react";
import { AlegreyaBold, SatoshiBold } from "@styles/fonts";

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