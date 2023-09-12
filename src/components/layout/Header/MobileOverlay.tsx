import React from "react";
import Container from "@mui/material/Container";
import Box from "@mui/material/Box";
import { HeaderBurgerMenu, HeaderButtons } from "./HeaderElements";
import Link from "next/link";
import { Text } from "../../Text/TextComponent";
import palette from "../../../styles/palette.module.scss";

interface MobileOverlayProps {
    setOpenMenu: React.Dispatch<React.SetStateAction<boolean>>;
}
export function MobileOverlay({ setOpenMenu }: MobileOverlayProps) {
    const fontSize = "20px";

    // There is a 10px width diff due to the scrollbar
    return (
        <Box
            className="flex flex-col items-center text-center overflow-hidden absolute"
            component="div"
            width="100vw"
            height="100vh"
            top="0"
            left="0"
            position="fixed"
            bgcolor={palette.baseYellow}
            rowGap={"12px"}
            zIndex={20}
        >
            {/* TOP ROW */}
            <Container sx={{ display: "flex", justifyContent: "flex-start", paddingY: "8px" }}>
                {/* HEADER LOGO */}
                {/*<HeaderLogo />*/}

                {/* HAMBURGER MENU */}
                <HeaderBurgerMenu mobile={true} openMenuStateFun={() => setOpenMenu(false)} />
            </Container>

            {/* Links */}
            <Link href="/">
                <Text.Body1 color={"text.secondary"} sx={{ fontSize }}>
                    Home
                </Text.Body1>
            </Link>

            {/* Socials */}
            <HeaderButtons/>
        </Box>
    );
}

interface useMobileOverlayProps {
    openMenu: boolean;
    setOpenMenu: React.Dispatch<React.SetStateAction<boolean>>;
    OverlayComponent: () => React.ReactNode;
    TriggerButton: () => React.ReactNode;
}
export function useMobileOverlay(): useMobileOverlayProps {
    const [openMenu, setOpenMenu] = React.useState<boolean>(false);

    const OverlayComponent = () => <>
        {openMenu && <MobileOverlay setOpenMenu={setOpenMenu} />}
    </>;

    const TriggerButton = () => <HeaderBurgerMenu openMenuStateFun={() => setOpenMenu(true)} />;

    return {
        openMenu,
        setOpenMenu,
        OverlayComponent,
        TriggerButton
    };
}