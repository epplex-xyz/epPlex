import Box from "@mui/material/Box";
import style from "../../styles/style.module.scss";
import { Text } from "@components/Text/TextComponent";
import React from "react";

export function ContainedContainer({children}) {
    return (
        <Box
            component="div"
            position="relative"
            rowGap={"16px"}
            display={"flex"}
            alignSelf={"center"}
            padding={"8px 16px"}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
        >
            <Text.Body1 fontSize={"14px"}>
                {children}
            </Text.Body1>
        </Box>
    );
}