// Main inspiration: https://www.youtube.com/watch?v=9paBIA2R5C0
import React from "react";
import Container from "@mui/material/Container";
import Toolbar from "@mui/material/Toolbar";
import Box from "@mui/material/Box";
import AppBar from "@mui/material/AppBar";
import {useMobileOverlay } from "./MobileOverlay";
import {HeaderBurgerMenu, HeaderLogo, HeaderButtons,} from "./HeaderElements";
import { ButtonLink } from "@components/Buttons/LinkButton";
import { ButtonConfig } from "@components/Buttons/ButtonConfig";

function HeaderLeft({TriggerButton}: {TriggerButton: () => React.ReactNode}) {
    return (
        <Box
            component="div"
            display="flex"
            justifyContent={"flex-start"}
            flex={"1 0 0"}
        >
            {/* Either show Logo or hamburger*/}
            <Box
                component="div"
                display={{ xs: "none", md: "flex"}}
            >
                <HeaderLogo />
            </Box>

            <Box
                component="div"
                display={{ xs: "flex", md: "none" }}
            >
                <TriggerButton/>
            </Box>
        </Box>
    );
}
function HeaderCenter() {
    return (
        <Box
            component="div"
            display="flex"
            justifyContent={"center"}
            flexGrow={1}
        >
            <Box component="div"
                display={{ xs: "none", sm: "none", md: "flex", lg: "flex" }}
            >
                <HeaderButtons />
            </Box>
        </Box>
    );
}


function HeaderRight() {
    return (
        <Box
            component="div"
            display="flex"
            justifyContent={"flex-end"}
            flex={"1 0 0"}
        >
            <ButtonLink {...ButtonConfig.app}/>
        </Box>
    );
}


export function Header({ headerPosition }) {
    const {OverlayComponent, TriggerButton} = useMobileOverlay();

    return (
        <>
            <AppBar
                color={"transparent"}
                sx={{
                    boxShadow: "unset",
                    zIndex: 10,
                    borderBottom: "1px solid #E4EBFF",
                    backdropFilter: "blur(10px)"
                }}
                position={headerPosition}
            >
                <Container className="!max-w-none !mx-0">
                    <Toolbar
                        disableGutters
                        sx={{
                            marginY: "8px",
                            display: "flex",
                            justifyContent: "space-between",
                        }}
                    >
                        <HeaderLeft TriggerButton={TriggerButton}/>

                        <HeaderCenter/>

                        <HeaderRight/>
                    </Toolbar>
                </Container>
            </AppBar>

            <OverlayComponent/>
        </>
    );
}