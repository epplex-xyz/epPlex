import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { Ephemerality } from "../target/types/ephemerality";
import { EphemeralMetadata } from "../target/types/ephemeral_metadata";

export function testPrelude() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Ephemerality as Program<Ephemerality>
    const metadata_program = anchor.workspace.EphemeralMetadata as Program<EphemeralMetadata>
    const connection = program.provider.connection;
    const wallet = (program.provider as anchor.AnchorProvider).wallet;

    return {
        program,
        metadata_program,
        connection,
        wallet,
    };
}