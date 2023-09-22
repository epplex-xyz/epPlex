import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { Text } from "@components/Text/TextComponent";
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import { useProgramApis } from "../../providers/ProgramApisProvider";
import {AnchorProvider} from "@coral-xyz/anchor";
import { getToken22 } from "../../utils/solana";
import { Token22 } from "../../../client/token22";
import style from "../../styles/style.module.scss";
import Image from "next/image";
import { Timer } from "@components/Text/Timer";
// JG2sDKq9r3Q2HPzzJom6kXSuFZRB5LRFofW7f5xoCMy

export function EpNFTContainer({item}: {item: Token22}) {
    const [image, setImage] = useState<string>("");

    const fetchImage = useCallback(async () => {
        try {
            const response = await fetch(item.uri).then((response) => response.json());

            setImage(response.image);
            // const objectURL = URL.createObjectURL(blob);
            // console.log("EpNFTContainer", objectURL);
        } catch (e) {
            console.log("Failed to fetch image", e);
        }
    }, []);

    useEffect(() => {
        fetchImage().then();
    },[]);

    return (
        <Box
            component="div"
            position="relative"
            flexDirection={"column"}
            rowGap={"16px"}
            alignItems={"center"}
            display={"flex"}
            height={"800px"}
            alignSelf={"start"}
            width={{ sm: "300px", md: "400px" }}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
        >
            <Timer endTimestamp={Number(item.destroyTimestampValue)}/>

            <Image
                src={image}
                alt={"logo"}
                height={300}
                width={300}
            />
        </Box>
    );
}
