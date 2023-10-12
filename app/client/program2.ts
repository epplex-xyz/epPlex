import {
    Connection,
    Keypair, ParsedAccountData,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram, SYSVAR_RENT_PUBKEY, Transaction,
} from "@solana/web3.js";
import { createProgram, EphemeralityProgram } from "./types/programTypes";
import {AnchorProvider, Wallet} from "@coral-xyz/anchor";
import { getMintOwner, mintToIx, sendAndConfirmRawTransaction, tryCreateATAIx2 } from "../utils/solana";
import { CONFIRM_OPTIONS } from "./constants";
import {
    ExtensionType, getAssociatedTokenAddressSync,
    getMintLen,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import {BN} from "@coral-xyz/anchor";
import { AnchorWallet} from "@solana/wallet-adapter-react";

export class Program2 {
    connection: Connection;
    program: EphemeralityProgram;

    wallet: Wallet;


    constructor(
        wallet: AnchorWallet,
        connection: Connection,
    ) {
        const provider = new AnchorProvider(connection, wallet, CONFIRM_OPTIONS);
        this.program = createProgram(provider);
        this.connection = connection;
        this.wallet = (this.program.provider as AnchorProvider).wallet as Wallet;
    }

    async createToken(
        mint: Keypair,
        destroyTimestampOffset: number = 60 * 5,
        name: string = "Ephemeral burger",
        symbol: string = "EP",
        uri: string = "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo",
    ) {
        const METADATAPOINTER_SIZE = 64 + 2 + 2;
        const programDelegate = this.getProgramDelegate();
        const payer = this.wallet.publicKey;

        const tokenCreateIx = await this.program.methods
            .tokenCreate({
                destroyTimestampOffset: new BN(destroyTimestampOffset),
                name: name,
                symbol: symbol,
                uri: uri,
            })
            .accounts({
                mint: mint.publicKey,
                programDelegate: programDelegate,
                payer: payer,
                systemProgram: SystemProgram.programId,
                token22Program: TOKEN_2022_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            })
            .instruction();

        const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
        const mintLen = getMintLen(extensions) + METADATAPOINTER_SIZE;
        const mintLamports = await this.connection.getMinimumBalanceForRentExemption(mintLen);

        const transaction = new Transaction().add(...[
            SystemProgram.createAccount({
                fromPubkey: payer,
                newAccountPubkey: mint.publicKey,
                space: mintLen,
                lamports: mintLamports,
                programId: TOKEN_2022_PROGRAM_ID,
            }),
            tokenCreateIx,
            ...mintToIx(mint.publicKey, payer)
        ]);

        let tx;
        try {
            tx = await sendAndConfirmRawTransaction(this.connection, transaction, payer, this.wallet, [mint]);
            console.log("tx", tx);
        } catch (e) {
            console.log("Failed to send tx", e);
        }
        return tx;
    }

    async burnToken(
        mint: PublicKey,
        // payer: Keypair
    ) {
        const programDelegate = this.getProgramDelegate();
        const mintOwner = await getMintOwner(this.connection, mint);
        const ata = getAssociatedTokenAddressSync(
            mint,
            mintOwner,
            undefined,
            TOKEN_2022_PROGRAM_ID
        );

        const tokenBurnTx = await this.program.methods
            .tokenBurn({})
            .accounts({
                mint: mint,
                programDelegate: programDelegate,
                tokenAccount: ata,
                token22Program: TOKEN_2022_PROGRAM_ID,
            })
            .transaction();

        const tx = await sendAndConfirmRawTransaction(
            this.connection,
            tokenBurnTx,
            this.wallet.publicKey,
            this.wallet,
            []
        );

        return tx;
    }


    getProgramDelegate(): PublicKey {
        const [programDelegate] = PublicKey.findProgramAddressSync(
            [Buffer.from("PROGRAM_DELEGATE")],
            this.program.programId
        );
        return programDelegate;
    }

}