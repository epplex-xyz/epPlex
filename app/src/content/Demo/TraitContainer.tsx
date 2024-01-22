import React from "react";
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import style from "../../styles/style.module.scss";

export function TraitContainer({ trait, value }: { trait: string; value: string }) {
    return (
        <Box
            component="div"
            position="relative"
            display={"flex"}
            flexDirection={"column"}
            alignSelf={"center"}
            color={"primary.main"}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
            padding={"8px 16px"}
        >
            <Text.Body2 fontSize={"10px"} className="whitespace-nowrap">
                {trait}:
            </Text.Body2>
            <Text.Body2 fontSize={"10px"} className="whitespace-nowrap">
                {value}
            </Text.Body2>
        </Box>
    );
}

