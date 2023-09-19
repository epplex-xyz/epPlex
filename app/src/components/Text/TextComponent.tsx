import React, { useMemo } from "react";
import Typography, { TypographyProps } from "@mui/material/Typography";
import {
    FontVariantKey,
    TextBackground,
    TypographyVariant,
    TypographyVariantsKey,
    typography,
    useFontFamily,
    useTextShadowColor,
} from "./Typography";

interface TextProps extends TypographyProps {
    variant: TypographyVariantsKey;
    fontVariant?: FontVariantKey;
    children?: React.ReactNode;
    color?: string;
    textBackground?: TextBackground;
    textShadow?: boolean;
}
export function Text({
    variant,
    fontVariant = "normal",
    color,
    textBackground = TextBackground.red,
    textShadow = true,
    children,
    ...props
}: TextProps) {
    const fontFamily = useFontFamily({ typographyVariant: variant, fontVariant });
    const variantStyle: TypographyVariant = useMemo(() => {
        const style = typography[variant];
        return {
            ...style,
            color: color ?? style.color,
        };
    }, [color]);
    return (
        <Typography
            variant={variant}
            fontFamily={fontFamily}
            color={variantStyle.color}
            {...props}
        >
            {children}
        </Typography>
    );
}

function H1(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h1" color={color}>
            {children}
        </Text>
    );
}

function H2(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h2" color={color}>
            {children}
        </Text>
    );
}

function H3(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h3" color={color}>
            {children}
        </Text>
    );
}

function H4(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h4" color={color}>
            {children}
        </Text>
    );
}

function H5(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h5" color={color}>
            {children}
        </Text>
    );
}

function H6(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="h6" color={color}>
            {children}
        </Text>
    );
}

function Subtitle1(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="subtitle1" color={color}>
            {children}
        </Text>
    );
}

function Body1(props: Omit<TextProps, "variant">) {
    const { color, children } = props;
    return (
        <Text {...props} variant="body1" color={color}>
            {children}
        </Text>
    );
}

function Body2(props: Omit<TextProps, "variant">) {
    const { color = "text.primary", children } = props;
    return (
        <Text {...props} variant="body2" color={color}>
            {children}
        </Text>
    );
}

Text.H1 = H1;
Text.H2 = H2;
Text.H3 = H3;
Text.H4 = H4;
Text.H4 = H4;
Text.H5 = H5;
Text.H6 = H6;
Text.Subtitle1 = Subtitle1;
Text.Body1 = Body1;
Text.Body2 = Body2;
