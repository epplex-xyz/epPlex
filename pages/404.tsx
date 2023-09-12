import React from "react";
import Screen from "../src/components/layout/Screen";
import { useRouter } from "next/router";
import style from "@styles/style.module.scss";
import { Text } from "../src/components/Text/TextComponent";

export default function PageNotFound() {
    const router = useRouter();

    return (
        <Screen>
            <div style={{ height: style.viewportHeight }} className="w-screen flex items-center justify-center">
                <Text.H6
                    sx={{ textAlign: "center", marginX: "16px" }}
                >
                    Oops! Not yet implemented: <i>{router.asPath}</i>
                </Text.H6>
            </div>
        </Screen>
    );
}
