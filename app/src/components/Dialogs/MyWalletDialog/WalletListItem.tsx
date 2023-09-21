import type { ListItemProps } from "@mui/material";
import { Button, ListItem } from "@mui/material";
import type { Wallet } from "@solana/wallet-adapter-react";
import type { FC, MouseEventHandler } from "react";
import React from "react";
import { WalletIcon } from "@solana/wallet-adapter-material-ui";
import Box from "@mui/material/Box";
import { WalletReadyState } from "@solana/wallet-adapter-base";
import { SatoshiBold } from "@styles/fonts";
import { Text } from "@components/Text/TextComponent";

interface WalletListItemProps extends Omit<ListItemProps, "onClick" | "button"> {
    onClick: MouseEventHandler<HTMLButtonElement>;
    wallet: Wallet;
}

export const WalletListItem: FC<WalletListItemProps> = ({ onClick, wallet, ...props }) => {
    const installedButtonProps = {
        onClick: onClick,
    };

    const uninstalledButtonProps = {
        href: wallet.adapter.url,
        target: "_blank",
    };

    const buttonProps =
        wallet.readyState === WalletReadyState.Installed ? installedButtonProps : uninstalledButtonProps;

    return (
        <ListItem {...props}>
            <Button {...buttonProps}>
                <div style={{ columnGap: "8px", display: "flex", alignItems: "center", fontSize: "16px" }}>
                    <WalletIcon wallet={wallet} />
                    <Text.H6 color={"text.secondary"}>
                        {wallet.adapter.name}
                    </Text.H6>
                </div>
                <div>
                    <Text.Body2 color={"text.secondary"}>
                        {wallet.readyState === WalletReadyState.Installed ? "INSTALLED" : "REDIRECT"}
                    </Text.Body2>
                </div>
            </Button>
        </ListItem>
    );
};
