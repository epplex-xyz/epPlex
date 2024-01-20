import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { EpplexCore } from "../target/types/epplex_core";
import { EpplexMetadata } from "../target/types/epplex_metadata";
import { EpplexMint } from "../target/types/epplex_mint";
import { EpplexBurger } from "../target/types/epplex_burger";

export function testPrelude() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const coreProgram = anchor.workspace.EpplexCore as Program<EpplexCore>
    const metadataProgram = anchor.workspace.EpplexMetadata as Program<EpplexMetadata>
    const mintProgram = anchor.workspace.EpplexMint as Program<EpplexMint>
    const burgerProgram = anchor.workspace.EpplexBurger as Program<EpplexBurger>
    const connection = coreProgram.provider.connection;
    const wallet = (coreProgram.provider as anchor.AnchorProvider).wallet;

    return {
        coreProgram,
        metadataProgram,
        mintProgram,
        burgerProgram,
        connection,
        wallet,
    };
}