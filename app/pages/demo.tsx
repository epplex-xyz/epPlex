import React from "react";
import Screen from "../src/components/layout/Screen";
import { MyWalletConnectButton } from "@components/Buttons/MyWalletConnectButton";
import { Section } from "@components/Container/Section";
import { WalletConnectedWrapper } from "@components/Container/WalletConnectedWrapper";
import { DemoPage } from "../src/content/Demo/DemoPage";
import MyWalletProvider from "../src/providers/MyWalletProvider";
import ProgramApisProvider from "../src/providers/ProgramApisProvider";

export default function Demo() {
    const notConnectWrapper = <MyWalletConnectButton>
        Connect wallet to create epNFTs
    </MyWalletConnectButton>;

    return (
        <MyWalletProvider>
            <ProgramApisProvider>
                <Screen headerPosition={"static"}>
                    <Section>
                        <WalletConnectedWrapper wrapper={notConnectWrapper}>
                            <DemoPage/>
                        </WalletConnectedWrapper>
                    </Section>
                </Screen>
            </ProgramApisProvider>
        </MyWalletProvider>
    );
}
