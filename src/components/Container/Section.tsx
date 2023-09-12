import React from "react";
import Box, { BoxProps } from "@mui/material/Box";

export function Section({ children, ref, overflow="hidden", height="100vh", ...props }: BoxProps) {
    return (
        <Box

            {...props}
            ref={ref}
            component="div"
            overflow={overflow}
            width={"100%"}
            height={height}
            position={"relative"}
            display={"flex"}
            justifyContent={"center"}
            alignItems={"center"}
        >
            {children}
        </Box>
    );
}
