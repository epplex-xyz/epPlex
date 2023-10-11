import {FileCopy as CopyIcon, LinkOff as DisconnectIcon, SwapHoriz as SwitchIcon} from '@mui/icons-material';
import {Button, ButtonProps, Collapse, Fade, ListItemIcon, Menu, MenuItem, Theme} from '@mui/material';
import {styled} from "@mui/material/styles";
import {useWallet, Wallet} from '@solana/wallet-adapter-react';
import React, {FC, useMemo, useState} from 'react';
import {WalletConnectButton, WalletIcon} from "@solana/wallet-adapter-material-ui";
import Link from "next/link";
import AccountCircleIcon from '@mui/icons-material/AccountCircle';
import {useWalletDialog} from "../Dialogs/MyWalletDialog/MyWalletDialogProvider";
import {WalletDialogButton} from "../Dialogs/MyWalletDialog/WalletDialogButton";
import {PublicKey} from "@solana/web3.js";
import LunchDiningIcon from '@mui/icons-material/LunchDining';
import { useIsMounted } from "../../hooks/useIsMounted";

const StyledMenu = styled(Menu)(({ theme }: { theme: Theme }) => ({
    '& .MuiList-root': {
        padding: 0,
    },
    '& .MuiListItemIcon-root': {
        marginRight: theme.spacing(),
        minWidth: 'unset',
        '& .MuiSvgIcon-root': {
            width: 20,
            height: 20,
        },
    },
    '& .MuiPopover-paper': {
        width: "180px"
    },
}));

const WalletActionMenuItem = styled(MenuItem)(({ theme }: { theme: Theme }) => ({
    padding: theme.spacing(1, 2),
    color: theme.palette.text.secondary,
    boxShadow: 'inset 0 1px 0 0 ' + 'rgba(255, 255, 255, 0.1)',
    '&:hover': {
        boxShadow: 'inset 0 1px 0 0 ' + 'rgba(255, 255, 255, 0.1)' + ', 0 1px 0 0 ' + 'rgba(255, 255, 255, 0.05)',
    },
}));

const WalletMenuItem = styled(WalletActionMenuItem)(({ theme }: { theme: Theme }) => ({
    padding: 0,
    '& .MuiButton-root': {
        borderRadius: 0,
    },
    // To fix small part sticking out on mobile view for the phantom connect button
    [theme.breakpoints.down('sm')]: {
        minHeight: "unset"
    },
}));

export const MyWalletConnectButton: FC<ButtonProps> = ({
    color = 'primary',
    variant = 'contained',
    type = 'button',
    children,
    startIcon = <></>,
    ...props
}) => {
    let publicKey: PublicKey | null, wallet: Wallet | null, disconnect: () => Promise<void>;
    const w = useWallet();

    publicKey = w?.publicKey;
    wallet = w?.wallet;
    disconnect = w?.disconnect;

    const { setOpen } = useWalletDialog();
    const [anchor, setAnchor] = useState<HTMLElement>();

    const base58 = useMemo(() => publicKey?.toBase58(), [publicKey]);
    const content = useMemo(() => {
        if (children === "") return children;
        if (!wallet || !base58) return null;
        if (children === "My Profile") return children + " (" + base58.slice(0, 4) + ")";
        return base58.slice(0, 4) + '..' + base58.slice(-4);
    }, [children, wallet, base58]);

    if (!wallet) {
        return (
            <WalletDialogButton color={color} variant={variant} type={type} startIcon={startIcon} {...props} >
                {children}
            </WalletDialogButton>
        );
    }
    if (!base58) {
        return (
            <WalletConnectButton color={color} variant={variant} type={type} startIcon={startIcon} {...props}>
                {children}
            </WalletConnectButton>
        );
    }

    return (
        <>
            <Button
                color={color}
                variant={variant}
                type={type}
                startIcon={<AccountCircleIcon/>}
                onClick={(event) => setAnchor(event.currentTarget)}
                aria-controls="wallet-menu"
                aria-haspopup="true"
                {...props}
            >
                {content}
            </Button>
            <StyledMenu
                id="wallet-menu"
                anchorEl={anchor}
                open={!!anchor}
                onClose={() => setAnchor(undefined)}
                marginThreshold={0}
                TransitionComponent={Fade}
                transitionDuration={250}
                keepMounted
                transformOrigin={{
                    vertical: "top",
                    horizontal: "center",
                }}
                anchorOrigin={{
                    vertical: 'top',
                    horizontal: 'center',
                }}
            >
                <WalletMenuItem
                    onClick={() => setAnchor(undefined)}
                    // disableRipple
                >
                    <Button
                        // disableRipple
                        color={color}
                        variant={variant}
                        sx={{
                            boxShadow: "unset",
                            // cursor: "default",
                            "&:hover": {
                                boxShadow: "unset",
                                // backgroundColor: (theme) => theme.palette.primary.main
                            }
                        }}
                        type={type}
                        startIcon={<WalletIcon wallet={wallet} />}
                        onClick={(event) => setAnchor(undefined)}
                        fullWidth
                        {...props}
                    >
                        {wallet.adapter.name}
                    </Button>
                </WalletMenuItem>
                <Collapse in={!!anchor}>
                    {/*<SingleDivider/>*/}
                    <WalletActionMenuItem
                        onClick={async () => {
                            setAnchor(undefined);
                            await navigator.clipboard.writeText(base58);
                        }}
                    >
                        <ListItemIcon>
                            <CopyIcon />
                        </ListItemIcon>
                        Copy address
                    </WalletActionMenuItem>
                    <WalletActionMenuItem
                        onClick={() => {
                            setAnchor(undefined);
                            setOpen(true);
                        }}
                    >
                        <ListItemIcon>
                            <SwitchIcon />
                        </ListItemIcon>
                        Change wallet
                    </WalletActionMenuItem>
                    <WalletActionMenuItem
                        onClick={() => {
                            setAnchor(undefined);
                            disconnect().catch(() => {
                                // Silently catch because any errors are caught by the context `onError` handler
                            });
                        }}
                    >
                        <ListItemIcon>
                            <DisconnectIcon />
                        </ListItemIcon>
                        Disconnect
                    </WalletActionMenuItem>
                </Collapse>
            </StyledMenu>
        </>
    );
};

export const MyMountedWalletButton: FC<ButtonProps> = ({children, ...props}) => {
    const isMounted = useIsMounted();

    return (<>
        {isMounted &&
            <MyWalletConnectButton {...props}>
                {children}
            </MyWalletConnectButton>
        }
    </>);
};
