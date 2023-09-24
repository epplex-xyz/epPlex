import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { TextDivider } from "@components/Divider/TextDivider";
import { Text } from "@components/Text/TextComponent";
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import { useProgramApis } from "../../providers/ProgramApisProvider";
import { getToken22 } from "../../../utils/solana";
import { Token22 } from "../../../client/types/token22";
import CircularProgress from '@mui/material/CircularProgress';
import { Carousel } from "./Carousel";
import { EpNFTContainer } from "./EpNFTContainer";
// JG2sDKq9r3Q2HPzzJom6kXSuFZRB5LRFofW7f5xoCMy

export function MyEpNFTs() {
    const [isFetching, setIsFetching] = useState<boolean>(true);
    const [tokens, setTokens] = useState<Token22[]>([]);
    const {program} = useProgramApis();

    const fetchNFTs = useCallback(async (program) => {
        setIsFetching(true);

        try {
            if (program.wallet !== undefined) {
                const tokens = await getToken22(program.connection, program.wallet.publicKey);
                setTokens(tokens);
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
            rowGap={"16px"}
            display={"flex"}
            alignSelf={"start"}
            width={{ sm: "300px", md: "400px" }}
        >
            <div className="absolute top-0 w-full">
                <TextDivider>My epNFTs</TextDivider>
            </div>

            <div className="flex justify-center self-center items-center w-full flex-col">
                {isFetching ? <CircularProgress sx={{color: "secondary.main"}} /> :
                    <>
                        { tokens.length === 0 ?
                            <>
                                <ChevronLeftIcon sx={{color: "secondary.main"}}/>
                                <Text.H6>
                                     Create an ephemeral NFT
                                </Text.H6>
                            </>
                            : <Carousel
                                items={tokens}
                                ItemComponent={EpNFTContainer}
                            />
                        }
                    </>
                }
            </div>
        </Box>
    );
}
