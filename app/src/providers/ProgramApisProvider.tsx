import React, { createContext, useContext, useMemo } from "react";
import { AnchorProvider } from "@coral-xyz/anchor";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { CONFIRM_OPTIONS } from "../../client/constants";
import { createProgram, EphemeralityProgram } from "../../client/types/programTypes";

export const useProgramApis = () => {
    const context = useContext(ProgramApisContext);
    if (context === undefined) {
        throw new Error("useProgramApis must be used within a ProgramApisProvider");
    }
    return context;
};

interface ProgramInterface {
    program: EphemeralityProgram;

}
const ProgramApisContext = createContext<ProgramInterface>({
    program: {} as EphemeralityProgram,
});

const ProgramApisProvider = ({ children }) => {
    const { connection } = useConnection();
    const anchorWallet = useAnchorWallet();

    const { program } = useMemo(() => {
        const provider = new AnchorProvider(connection, anchorWallet!, CONFIRM_OPTIONS);
        const program = createProgram(provider);

        return { program };
    }, [connection, anchorWallet]);

    return <ProgramApisContext.Provider value={{ program }}>{children}</ProgramApisContext.Provider>;
};

export default ProgramApisProvider;
