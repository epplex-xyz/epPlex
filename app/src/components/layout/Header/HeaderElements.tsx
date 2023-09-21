import React from "react";
import {IconButton} from "@mui/material";
import Link from "next/link";
import Image from "next/image";
import Box from "@mui/material/Box";
import { Text } from "../../Text/TextComponent";
import { TextBackground } from "../../Text/Typography";
import { ButtonLink } from "src/components/Buttons/LinkButton";

interface HeaderActionType {
    name: string
}
export const HeaderActions: Record<string, HeaderActionType> = {
    products: {
        name: "Products",
    },
    docs: {
        name: "Docs"
    },
    mission: {
        name: "Mission"
    },
    changeLog: {
        name: "Changelog"
    }
};

const HeaderActionsList = Object.values(HeaderActions).map(action => ({
    name: action.name
}));


export function HeaderLogo(){
    return (
        <Link href="/" style={{display: "flex", alignItems: "center"}}>
            <Image
                src={"/logos/newLogo.png"}
                alt="Home"
                width={50}
                height={50}
                priority={true}
            />
            <Text.H5
                fontVariant={"secondary"}
                textBackground={TextBackground.none}
                paddingLeft={"8px"}
                display={{ xs: "none", md: "none", lg: "flex", xl: "flex" }}
            >
                epPlex
            </Text.H5>
        </Link>
    );
}

export function HeaderBurgerMenu({openMenuStateFun, mobile = false}) {
    return (
        <IconButton onClick={openMenuStateFun}>
            <Image
                src={"/logos/newLogo.png"}
                alt="burgerMenu"
                width={46}
                height={46}
                priority={true}
            />
        </IconButton>
    );
}

export function HeaderButtons() {
    return (
        <Box
            component="div"
            className="flex gap-x-2 items-center"
        >
            {HeaderActionsList.map(({name}) => (
                <ButtonLink
                    key={`Header-${name}`}
                    href={`/${name.toLowerCase()}`}
                >
                    {name}
                </ButtonLink>
            ))}
        </Box>
    );
}


