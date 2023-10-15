import React from "react";
import { IconButton } from "@mui/material";
import Link from "next/link";
import Image from "next/image";
import { Text } from "../../Text/TextComponent";
import { TextBackground } from "../../Text/Typography";


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

