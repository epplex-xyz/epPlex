import React from "react";
import Box, { BoxProps } from "@mui/material/Box";
import style from "../../styles/style.module.scss";

export function Section({ children, ref, overflow="hidden", ...props }: BoxProps) {
    return (
        <Box
            {...props}
            ref={ref}
            component="div"
            // overflow={overflow}
            width={"100%"}
            minHeight={{sm: "100%", md: style.viewportHeight}}
            position={"relative"}
            display={"flex"}
            justifyContent={"center"}
            alignItems={"center"}
            marginBottom={"50px"}
        >
            {children}
        </Box>
    );
}
