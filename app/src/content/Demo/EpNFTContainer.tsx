import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import style from "../../styles/style.module.scss";
import Image from "next/image";
import { Timer } from "@components/Text/Timer";
import { ContainedContainer } from "@components/Container/ContainedContainer";
import Button from "@mui/material/Button";
import BombIcon from "../../../public/icons/bomb.svg";
import { useProgramApis } from "../../providers/ProgramApisProvider";
import CircularProgress from "@mui/material/CircularProgress";
import toast from "react-hot-toast";
import { ToastText } from "@components/Text/ToastText";
import {TokenMetadata} from  "@solana/spl-token-metadata";
import { TraitContainer } from "./TraitContainer";


export function EpNFTContainer({item}: {item: TokenMetadata}) {
    const [image, setImage] = useState<string>("");
    const [traitList, setTraitList] = useState<any[]>([]); // State for the list of trait objects

    console.log("item", item.name, item.symbol);
    // probably dont need to use this in this contaienr
    const {program, hasCreatedtState: {setHasCreated}} = useProgramApis();
    const [loading, setLoading] = React.useState(false);

    // const destroyTimestamp = Number(item.expirationDateValue);
    // const canDestroy = Math.floor(Date.now() / 1000) > destroyTimestamp;

    const fetchImage = useCallback(async () => {
        try {
            const response = await fetch(item.uri).then((response) => response.json());

            // Image
            setImage(response.image);

            // Traits
            const traitObjects = JSON.parse(JSON.stringify(response.attributes));
            if (Array.isArray(traitObjects)) {
                // Check if each item in the array has the required properties
                const isValid = traitObjects.every((traitObject) =>
                    Object.prototype.hasOwnProperty.call(traitObject, 'trait_type') &&
                    Object.prototype.hasOwnProperty.call(traitObject, 'value')
                );


                if (isValid) {
                    // Add the array of trait objects to the list
                    setTraitList([...traitObjects]);
                } else {
                    alert('Invalid input. Please provide an array of valid trait objects.');
                }
            } else {
                alert('Invalid input. Please provide an array of trait objects.');
            }
            // setImage(response.attributes);

        } catch (e) {
            console.log("Failed to fetch image", e);
        }
    }, []);

    useEffect(() => {
        fetchImage().then();
    },[]);


    const destroyNFT = useCallback(async (e) => {
        e.stopPropagation();
        setLoading(true);
        // try {
        //     const txId = await program.burnToken(item.metadataAddress);
        //     if (txId === "") {
        //         throw new Error("Failed to destroy");
        //     }
        //
        //     toast.success(
        //         <ToastText text={"Successfully destroyed epNFT:"} signature={txId}/>
        //     );
        //     setHasCreated((prev) => !prev);
        // } catch (e) {
        //     console.log("Failed to destroy", e);
        //     toast.error("Failed to destroy");
        // } finally {
        //     setLoading(false);
        // }
    }, [setHasCreated]);


    return (
        <Box
            component="div"
            position="relative"
            flexDirection={"column"}
            rowGap={"16px"}
            paddingY={"24px"}
            paddingX={"24px"}
            alignItems={"center"}
            display={"flex"}
            maxHeight={"800px"}
            width={"300px"}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
        >
            {/*<ContainedContainer>*/}
            {/*    <Timer endTimestamp={destroyTimestamp}/>*/}
            {/*</ContainedContainer>*/}

            { image &&
                <Image
                    src={image}
                    alt={"logo"}
                    height={200}
                    width={200}
                    style={{borderRadius: style.borderRadiusMd}}
                />
            }
            {/* Just using emtadataAddress for now*/}
            {/*<AddressCopy address={item.metadataAddress.toString()}/>*/}

            <div className="flex justify-between w-full items-center">
                <Text.Body1>
                    Name
                </Text.Body1>
                <ContainedContainer>
                    {item.name}
                </ContainedContainer>
            </div>

            <div className="flex justify-between w-full items-center">
                <Text.Body1>
                    Symbol
                </Text.Body1>
                <ContainedContainer>
                    {item.symbol}
                </ContainedContainer>
            </div>

            <div className="flex flex-row flex-wrap gap-y-2 gap-x-2">
                {traitList.map((trait, index) => (
                    <React.Fragment key={index}>
                        <TraitContainer trait={trait.trait_type} value={trait.value}/>
                    </React.Fragment>
                ))}
            </div>
            <div className={"flex w-full justify-center"}>
                <Button
                    variant={"contained"}
                    sx={{
                        marginTop: "16px",
                        columnGap: "8px"
                    }}
                    // disabled={!canDestroy}
                    onClick={destroyNFT}
                >
                    {loading ?
                        <>
                            Destroying <CircularProgress  size={"14px"} sx={{color: "text.secondary"}}/>
                        </>
                        : <>
                            Destroy <BombIcon/>
                        </>
                    }
                </Button>
            </div>
        </Box>
    );
}
