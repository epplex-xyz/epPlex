import React from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { Text } from "@components/Text/TextComponent";
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';

export function MyEpNFTs() {
    return (
        <Box
            component="div"
            position="relative"
            height={"100%"}
            // flexDirection="column"
            rowGap={"16px"}
            display={"flex"}
            alignSelf={"start"}
            // alignItems={"center"}
            // justifyContent={"center"}
            width={{ sm: "300px", md: "400px" }}
        >
            <div className="absolute top-0 w-full">
                <TextDivider>My epNFTs</TextDivider>
            </div>

            <div className="flex justify-center items-center w-full">
                <ChevronLeftIcon sx={{color: "secondary.main"}}/>
                <Text.H6>
                     Create an ephemeral NFT
                </Text.H6>
            </div>
        </Box>
    );
}
