import React from "react";
import {useMountedWallet} from "../../hooks/useIsMounted";

interface WalletConnectedWrappeProps {
    wrapper: React.ReactNode,
    children: React.ReactNode
}
export function WalletConnectedWrapper({
    wrapper, children
}: WalletConnectedWrappeProps) {
    const {mounted, connected} = useMountedWallet();

    return (
        <>
            {mounted &&
                <>
                    { connected ?
                        <>{children}</>
                        :
                        <>{wrapper}</>
                    }
                </>
            }
        </>
    );
}

