import React, { useCallback, useEffect, useState } from "react";
import Box from "@mui/material/Box";
import { Text } from "@components/Text/TextComponent";
import { Token22 } from "../../../client/types/token22";
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
            <ContainedContainer>
                <Timer endTimestamp={Number(item.destroyTimestampValue)}/>
            </ContainedContainer>

            { image &&
                <Image
                    src={image}
                    alt={"logo"}
                    height={200}
                    width={200}
                />
            }

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
        </Box>
    );
}
