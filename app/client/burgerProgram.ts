import { Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { CORE_PROGRAM_ID, createBurgerProgram, EpplexBurgerProgram } from "./types/programTypes";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { getMintOwner, sendAndConfirmRawTransaction } from "../utils/solana";
import { CONFIRM_OPTIONS } from "./constants";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { AnchorWallet } from "@solana/wallet-adapter-react";

export class BurgerProgram {
    connection: Connection;
    program: EpplexBurgerProgram;
    wallet: Wallet;

    constructor(wallet: AnchorWallet, connection: Connection) {
        const provider = new AnchorProvider(connection, wallet, CONFIRM_OPTIONS);
        this.program = createBurgerProgram(provider);
        this.connection = connection;
        this.wallet = (this.program.provider as AnchorProvider).wallet as Wallet;
    }

    async createWhitelistMint(
        destroyTimestamp: string,
        name: string = "Ephemeral burger",
        symbol: string = "EP",
        uri: string = "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo"
    ) {
        const permanentDelegate = this.getProgramDelegate();
        const payer = this.wallet.publicKey;
        const mint = Keypair.generate();
        const ata = getAssociatedTokenAddressSync(
            mint.publicKey,
            payer,
            undefined,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const tokenCreateTx = await this.program.methods
            .whitelistMint({
                name: name,
                symbol: symbol,
                uri: uri,
                destroyTimestamp: destroyTimestamp,
            })
            .accounts({
                mint: mint.publicKey,
                ata,
                // tokenMetadata: tm,
                permanentDelegate: permanentDelegate,
                payer: payer,
                systemProgram: SystemProgram.programId,
                token22Program: TOKEN_2022_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
                associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                epplexCore: CORE_PROGRAM_ID,
            })
            .transaction();

        let id;
        try {
            id = await sendAndConfirmRawTransaction(this.connection, tokenCreateTx, payer, this.wallet, [mint]);
            console.log("tx", id);
        } catch (e) {
            console.log("Failed to send tx", e);
        }
        return id;
    }

    async burnToken(mint: PublicKey) {
        const programDelegate = this.getProgramDelegate();
        const mintOwner = await getMintOwner(this.connection, mint);
        const ata = getAssociatedTokenAddressSync(mint, mintOwner, undefined, TOKEN_2022_PROGRAM_ID);

        const tokenBurnTx = await this.program.methods
            .tokenBurn({})
            .accounts({
                mint: mint,
                programDelegate: programDelegate,
                tokenAccount: ata,
                payer: this.wallet.publicKey,
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

    async createProgramDelegate() {
        const programDelegate = this.getProgramDelegate();

        const tx = await this.program.methods
            .programDelegateCreate({})
            .accounts({
                programDelegate,
                payer: this.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .transaction();

        const id = await sendAndConfirmRawTransaction(this.connection, tx, this.wallet.publicKey, this.wallet, []);
        console.log("tx", id);

        return id;
    }

    async renewToken(mint: PublicKey) {
        const ata = getAssociatedTokenAddressSync(mint, this.wallet.publicKey, undefined, TOKEN_2022_PROGRAM_ID);
        const programDelegate = this.getProgramDelegate();

        const tx = await this.program.methods
            .tokenRenew({ renewTerms: 1 })
            .accounts({
                mint,
                tokenAccount: ata,
                programDelegate,
                authority: this.wallet.publicKey,
                token22Program: TOKEN_2022_PROGRAM_ID,
            })
            .transaction();

        const id = await sendAndConfirmRawTransaction(this.connection, tx, this.wallet.publicKey, this.wallet, []);
        console.log("tx", id);

        return id;
    }

    getProgramDelegate(): PublicKey {
        const [programDelegate] = PublicKey.findProgramAddressSync(
            [Buffer.from("BURGER_DELEGATE")],
            this.program.programId
        );
        return programDelegate;
    }
}