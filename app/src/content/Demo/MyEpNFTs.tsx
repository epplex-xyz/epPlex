import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { Text } from "@components/Text/TextComponent";
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import { useProgramApis } from "../../providers/ProgramApisProvider";
import {AnchorProvider} from "@coral-xyz/anchor";
import { getTokenBalances } from "../../utils/solana";

export function MyEpNFTs() {
    // 6DoTJakcvoKwXougVGmwGkPWuB2pGLGXGNhwxTx46Rq
    const [isFetching, setIsFetching] = useState<boolean>(true);
    const {program} = useProgramApis();

    const fetchNFTs = useCallback(async (program) => {
        setIsFetching(true);

        try {
            const wallet = (program.provider as AnchorProvider).wallet;
            if (wallet?.publicKey !== undefined) {
                const profileTickets = await getTokenBalances(program.provider.connection, wallet.publicKey);
            }
        } catch (e) {
            console.log("Failed getting NFTs", e);
        } finally {
            setIsFetching(false);
        }
    }, []);

    useEffect(() => {
        fetchNFTs(program).then();
    }, [program, fetchNFTs]);

    return (
        <Box
            component="div"
            position="relative"
            height={"100%"}
            // flexDirection="column"
            rowGap={"16px"}
            display={"flex"}
            alignSelf={"start"}
            width={{ sm: "300px", md: "400px" }}
        >
            <div className="absolute top-0 w-full">
                <TextDivider>My epNFTs</TextDivider>
            </div>

            <div className="flex justify-center self-center items-center w-full">
                <ChevronLeftIcon sx={{color: "secondary.main"}}/>
                <Text.H6>
                     Create an ephemeral NFT
                </Text.H6>
            </div>
        </Box>
    );
}
