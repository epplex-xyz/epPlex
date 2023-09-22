import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import { Token22 } from "../../../client/token22";
import style from "../../styles/style.module.scss";
import Image from "next/image";
import { Timer } from "@components/Text/Timer";
import { ContainedContainer } from "@components/Container/ContainedContainer";

// JG2sDKq9r3Q2HPzzJom6kXSuFZRB5LRFofW7f5xoCMy

function TraitContainer({trait, value}: {trait: string, value: string}) {
    return (
        <Box
            component="div"
            position="relative"
            display={"flex"}
            flexDirection={"column"}
            alignSelf={"center"}
            color={"primary.main"}
            // bgcolor={"secondary.main"}
            sx={{
                borderRadius: style.borderRadiusMd,
                boxShadow: (theme) => `inset 0 0 0 1px ${theme.palette.text.primary}`,
            }}
            padding={"8px 16px"}
        >
            <Text.Body2 className="whitespace-nowrap">
                {trait}:
            </Text.Body2>
            <Text.Body2 className="whitespace-nowrap">
                {value}
            </Text.Body2>
        </Box>
    )
}
export function EpNFTContainer({item}: {item: Token22}) {
    const [image, setImage] = useState<string>("");
    const [traitList, setTraitList] = useState<any[]>([]); // State for the list of trait objects

    const fetchImage = useCallback(async () => {
        try {
            console.log("item.uri", item.uri);
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

    return (
        <Box
            component="div"
            position="relative"
            flexDirection={"column"}
            rowGap={"24px"}
            paddingY={"24px"}
            paddingX={"24px"}
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
            <ContainedContainer>
                <Timer endTimestamp={Number(item.destroyTimestampValue)}/>
            </ContainedContainer>

            <Image
                src={image}
                alt={"logo"}
                height={300}
                width={300}
            />

            {/*<div className="flex flex-col items-center px-2 gap-y-2">*/}
            <div className="flex justify-between w-full">
                <Text.H6>
                    Name
                </Text.H6>
                <ContainedContainer>
                    {item.name}
                </ContainedContainer>
            </div>

            <div className="flex justify-between w-full">
                <Text.H6>
                    Symbol
                </Text.H6>
                <ContainedContainer>
                    {item.symbol}
                </ContainedContainer>
            </div>

            <div className="flex flex-row flex-wrap gap-y-2 gap-x-2">
                {traitList.map((trait, index) => (
                    <React.Fragment key={index}>
                        <TraitContainer trait={trait.trait_type} value={trait.value}/>
                    </React.Fragment>
                    //
                    // <li key={index}>
                    //     Trait Type: {trait.trait_type}, Value: {trait.value}
                    // </li>
                ))}
            </div>
        </Box>
    );
}
