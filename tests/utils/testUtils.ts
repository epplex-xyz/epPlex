import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { EpplexCore } from "../../target/types/epplex_core";
import { EpplexBurger } from "../../target/types/epplex_burger";

export function testPrelude() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const coreProgram = anchor.workspace.EpplexCore as Program<EpplexCore>
    const burgerProgram = anchor.workspace.EpplexBurger as Program<EpplexBurger>
    const connection = coreProgram.provider.connection;
    const wallet = (coreProgram.provider as anchor.AnchorProvider).wallet;

    return {
        coreProgram,
        burgerProgram,
        connection,
        wallet,
    };
}