import React from "react";
import Screen from "../src/components/layout/Screen";
import { MyWalletConnectButton } from "@components/Buttons/MyWalletConnectButton";
import { Snow } from "@components/Animation/Snow";
import { LandingPage } from "../src/content/Home/LandingPage";
import { SectionConfig } from "../src/content/Home/sections";
import { Section } from "@components/Container/Section";
import { WalletConnectedWrapper, WalletConnectedWrapper2 } from "@components/Container/WalletConnectedWrapper";
import Button from "@mui/material/Button";
import { useMountedWallet } from "../src/hooks/useIsMounted";

export default function Demo() {
    const {mounted, connected} = useMountedWallet();
    return (
        <Screen>
            <Section>
                <WalletConnectedWrapper
                    wrapper={
                        <MyWalletConnectButton>
                            Connect wallet to create epNFTs
                        </MyWalletConnectButton>
                    }
                >
                    asd
                </WalletConnectedWrapper>
            </Section>
        </Screen>
    );
}
