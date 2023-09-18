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


    // getMetadata(mint: PublicKey): PublicKey {
    //     const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    //     const [programDelegate] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    //         TOKEN_METADATA_PROGRAM_ID
    //     );
    //     return programDelegate;
    // }

}