import React, { useCallback, useMemo } from "react";
import { WalletError } from "@solana/wallet-adapter-base";
import { ConnectionProvider, WalletProvider } from "@solana/wallet-adapter-react";
import {
    PhantomWalletAdapter,
    SolflareWalletAdapter,
    TorusWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import toast from "react-hot-toast";
import { CustomWalletDialogProvider } from "../components/Dialogs/MyWalletDialog/MyWalletDialogProvider";

import { COMMITMENT } from "../../client/constants";

const MyWalletProvider = ({ children }) => {
    // const currentEndpoint = undefined;

    const endpoint = useMemo(() => {
        // Probably this is unnecessary since it is already set within the default
        // if (currentEndpoint === undefined) {
        return process.env.NEXT_PUBLIC_SOLANA_RPC_HOST as string;

    }, [process.env.NEXT_PUBLIC_SOLANA_RPC_HOST]);

    // Not even necessary to use this
    // https://twitter.com/burger606/status/1649453569651736587?s=20
    const wallets = useMemo(
        () => [
            new PhantomWalletAdapter(),
            new SolflareWalletAdapter(),
            new TorusWalletAdapter(),        ],
        [endpoint]
    );

    const onError = useCallback((error: WalletError) => {
        toast.error(error.message ? `${error.name}: ${error.message}` : error.name);
    }, []);

    return (
        <ConnectionProvider endpoint={endpoint} config={{ commitment: COMMITMENT }}>
            <WalletProvider wallets={wallets} onError={onError} autoConnect={true}>
                <CustomWalletDialogProvider>
                    {children}
                </CustomWalletDialogProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};

export default MyWalletProvider;
