import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram, SYSVAR_RENT_PUBKEY, Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import { createProgram, EphemeralityProgram } from "../client/types/programTypes";
import {AnchorProvider, Wallet} from "@coral-xyz/anchor";
import { CONFIRM_OPTIONS } from "../client/constants";
import { ExtensionType, getMintLen, getOrCreateAssociatedTokenAccount, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import {BN} from "@coral-xyz/anchor";

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

    async createToken(mint: Keypair, payer: Keypair, destroyTimestampOffset: number = 60 * 5) {
        const METADATAPOINTER_SIZE = 64 + 2 + 2;
        const programDelegate = this.getProgramDelegate();

        const tokenCreateIx = await this.program.methods
            .tokenCreate({
                destroyTimestampOffset: new BN(destroyTimestampOffset),
                name: "Ephemeral burger",
                symbol: "EP",
                uri: "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo",
            })
            .accounts({
                mint: mint.publicKey,
                programDelegate: programDelegate,
                // metadata: metadata,
                payer: payer.publicKey,
                systemProgram: SystemProgram.programId,
                token22Program: TOKEN_2022_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
                // tokenMetadataProgram: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            })
            .instruction();

        const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
        const mintLen = getMintLen(extensions) + METADATAPOINTER_SIZE;
        const mintLamports = await this.connection.getMinimumBalanceForRentExemption(mintLen);

        const transaction = new Transaction().add(...[
            SystemProgram.createAccount({
                fromPubkey: payer.publicKey,
                newAccountPubkey: mint.publicKey,
                space: mintLen,
                lamports: mintLamports,
                programId: TOKEN_2022_PROGRAM_ID,
            }),
            tokenCreateIx,
        ]);

        const tx = await sendAndConfirmTransaction(this.connection, transaction, [payer, mint], CONFIRM_OPTIONS);
        console.log("tx", tx);
    }

    async burnToken(mint: PublicKey, payer: Keypair) {
        const programDelegate = this.getProgramDelegate();
        const account = await getOrCreateAssociatedTokenAccount(
            this.connection,
            payer,
            mint,
            payer.publicKey,
            undefined,
            undefined,
            undefined,
            TOKEN_2022_PROGRAM_ID
        );

        const tokenBurnTx = await this.program.methods
            .tokenBurn({})
            .accounts({
                mint: mint,
                programDelegate: programDelegate,
                tokenAccount: account.address,
                token22Program: TOKEN_2022_PROGRAM_ID,
            })
            .transaction();

        const tx = await sendAndConfirmTransaction(this.connection, tokenBurnTx, [payer], CONFIRM_OPTIONS);
        console.log("tx", tx);
    }

    async closeProgramDelegate() {
        const programDelegate = this.getProgramDelegate();

        const tokenBurnTx = await this.program.methods
            .programDelegateClose({})
            .accounts({
                programDelegate: programDelegate,
                payer: this.signer.publicKey,
            })
            .transaction();

        const tx = await sendAndConfirmTransaction(this.connection, tokenBurnTx, [this.signer], CONFIRM_OPTIONS);
        console.log("tx", tx);
    }

    async initProgramDelegate(): Promise<TransactionInstruction> {
        const programDelegate = this.getProgramDelegate();
        const initDelegateIx = await this.program.methods
            .programDelegateCreate({})
            .accounts({
                programDelegate,
                payer: this.signer.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .instruction();
        return initDelegateIx;
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