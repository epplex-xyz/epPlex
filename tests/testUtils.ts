import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { EpplexCore } from "../target/types/epplex_core";
import { EpplexMetadata } from "../target/types/epplex_metadata";

export function testPrelude() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.EpplexCore as Program<EpplexCore>
    const metadata_program = anchor.workspace.EpplexMetadata as Program<EpplexMetadata>
    const connection = program.provider.connection;
    const wallet = (program.provider as anchor.AnchorProvider).wallet;

    return {
        program,
        metadata_program,
        connection,
        wallet,
    };
}