import React from "react";
import Screen from "../src/components/layout/Screen";
import { Main } from "../src/content/Home/Main";
import useMediaQuery from "@mui/material/useMediaQuery";
import { useTheme } from "@mui/material/styles";

export default function Index() {
    const isMobile = useMediaQuery(useTheme().breakpoints.down('sm'));

    return (
        <Screen headerPosition={isMobile ? "static" : "relative"}>
            <Main/>
        </Screen>
    );
}
