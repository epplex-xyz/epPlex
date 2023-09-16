import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { createProgram, EphemeralityProgram } from "../client/types/programTypes";
import {AnchorProvider, Wallet} from "@coral-xyz/anchor";
import { CONFIRM_OPTIONS } from "../client/constants";

export class Program {
    signer: Keypair;
    connection: Connection;
    program: EphemeralityProgram;

    constructor(
        signer: Keypair,
        connection: Connection,
    ) {
        const provider = new AnchorProvider(connection, new Wallet(signer), CONFIRM_OPTIONS);
        this.program = createProgram(provider);
        this.signer = signer;
        this.connection = connection;
    }

    getProgramDelegate(): PublicKey {
        const [programDelegate] = PublicKey.findProgramAddressSync(
            [Buffer.from("PROGRAM_DELEGATE")],
            this.program.programId
        );
        return programDelegate;
    }

}