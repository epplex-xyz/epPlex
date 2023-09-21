import React from "react";
import Screen from "../src/components/layout/Screen";
import { MyWalletConnectButton } from "@components/Buttons/MyWalletConnectButton";
import { Section } from "@components/Container/Section";
import { WalletConnectedWrapper } from "@components/Container/WalletConnectedWrapper";
import { DemoPage } from "../src/content/Demo/DemoPage";

export default function Demo() {
    const notConnectWrapper = <MyWalletConnectButton>
        Connect wallet to create epNFTs
    </MyWalletConnectButton>;

    return (
        <Screen headerPosition={"static"}>
            <Section>
                <WalletConnectedWrapper wrapper={notConnectWrapper}>
                    <DemoPage/>
                </WalletConnectedWrapper>
            </Section>
        </Screen>
    );
}
