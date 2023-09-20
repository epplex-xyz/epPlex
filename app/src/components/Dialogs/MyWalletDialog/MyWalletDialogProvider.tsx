import type { FC, ReactNode } from 'react';
import React, {createContext, useContext, useState} from 'react';
import type { WalletDialogProps } from './WalletDialog';
import { WalletDialog } from './WalletDialog';
import {styled} from "@mui/material/styles";
import style from "../../../styles/style.module.scss";

export interface WalletDialogContextState {
    open: boolean;
    setOpen: (open: boolean) => void;
}

export const WalletDialogContext = createContext<WalletDialogContextState>({} as WalletDialogContextState);

export function useWalletDialog(): WalletDialogContextState {
    return useContext(WalletDialogContext);
}

export interface WalletDialogProviderProps extends WalletDialogProps {
    children: ReactNode;
}

const MyWalletDialogProvider: FC<WalletDialogProviderProps> = ({ children, ...props }) => {
    const [open, setOpen] = useState(false);

    return (
        <WalletDialogContext.Provider
            value={{
                open,
                setOpen
            }}
        >
            {children}
            <WalletDialog {...props} />
        </WalletDialogContext.Provider>
    );
};

export const CustomWalletDialogProvider = styled(MyWalletDialogProvider)(({ theme }) => ({
    "& .MuiPaper-root": {
        borderRadius: style.borderRadiusMd, // literally had to try all Mui styling options
    },
    "& .MuiList-root": {
        background: "none !important",
    },
    "& .MuiListItem-root": {
        boxShadow: "inset 0 1px 0 0 rgb(0 0 0 / 10%) !important",
    },
}));
