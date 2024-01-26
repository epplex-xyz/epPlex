import {
    ComputeBudgetProgram,
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    Transaction, TransactionInstruction,
} from "@solana/web3.js";
import { CORE_PROGRAM_ID, createBurgerProgram, EpplexBurgerProgram, VAULT } from "./types/programTypes";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { getMintOwner, sendAndConfirmRawTransaction, tryCreateATAIx } from "../utils/solana";
import { CONFIRM_OPTIONS } from "./constants";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { AnchorWallet } from "@solana/wallet-adapter-react";
import { NativeMint } from "@switchboard-xyz/solana.js";

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
        mint: Keypair = Keypair.generate(),
        name: string = "Ephemeral burger",
        symbol: string = "EP",
        uri: string = "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo"
    ) {
        const permanentDelegate = this.getProgramDelegate();
        const payer = this.wallet.publicKey;
        const ata = getAssociatedTokenAddressSync(
            mint.publicKey,
            payer,
            undefined,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        const tokenCreateIx = await this.program.methods
            .whitelistMint({
                name: name,
                symbol: symbol,
                uri: uri,
                expiryDate: destroyTimestamp,
            })
            .accounts({
                mint: mint.publicKey,
                tokenAccount: ata,
                tokenMetadata: this.getTokenBurgerMetadata(mint.publicKey),
                permanentDelegate: permanentDelegate,
                payer: payer,

                rent: SYSVAR_RENT_PUBKEY,
                systemProgram: SystemProgram.programId,
                token22Program: TOKEN_2022_PROGRAM_ID,
                associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                epplexCore: CORE_PROGRAM_ID,
            })
            .instruction();

        const ixs = [
            // prolly could tweak this further down
            ComputeBudgetProgram.setComputeUnitLimit({ units: 250_000 }),
            tokenCreateIx
        ];

        const tokenCreateTx = new Transaction().add(...ixs);
        let id;
        try {
            id = await sendAndConfirmRawTransaction(this.connection, tokenCreateTx, payer, this.wallet, [mint]);
            console.log("tx", id);
        } catch (e) {
            console.log("Failed to send tx", e);
        }
        return id;
    }

    // async burnToken(mint: PublicKey) {
    //     const programDelegate = this.getProgramDelegate();
    //     const mintOwner = await getMintOwner(this.connection, mint);
    //     const ata = getAssociatedTokenAddressSync(mint, mintOwner, undefined, TOKEN_2022_PROGRAM_ID);
    //
    //     const tokenBurnTx = await this.program.methods
    //         .tokenBurn({})
    //         .accounts({
    //             mint: mint,
    //             programDelegate: programDelegate,
    //             tokenAccount: ata,
    //             payer: this.wallet.publicKey,
    //             token22Program: TOKEN_2022_PROGRAM_ID,
    //         })
    //         .transaction();
    //
    //     const tx = await sendAndConfirmRawTransaction(
    //         this.connection,
    //         tokenBurnTx,
    //         this.wallet.publicKey,
    //         this.wallet,
    //         []
    //     );
    //
    //     return tx;
    // }

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
        // const ata = getAssociatedTokenAddressSync(
        //     mint, this.wallet.publicKey, undefined, TOKEN_2022_PROGRAM_ID
        // );

        const ixs: TransactionInstruction[] = [];

        const switchboardMint: NativeMint = await NativeMint.load(this.program.provider as AnchorProvider);
        const [payerAta, wrapSolTxn] = await switchboardMint.getOrCreateWrappedUserInstructions(
            this.wallet.publicKey, { fundUpTo: 1.1}
        );
        if (wrapSolTxn === undefined) {
            throw new Error("Wrap SOL failed");
        }
        ixs.push(...wrapSolTxn.ixns);

        // VAULT Ata
        const proceedsAta = getAssociatedTokenAddressSync(
            NativeMint.address, VAULT, undefined, TOKEN_PROGRAM_ID
        );

        // Payer Ata
        // const payerAta = getAssociatedTokenAddressSync(
        //     NativeMint.address, this.wallet.publicKey, undefined, TOKEN_PROGRAM_ID
        // );
        // const payerIx = await tryCreateATAIx(
        //     this.connection, this.wallet.publicKey, payerAta, this.wallet.publicKey, NativeMint.address
        // );

        const proceedsIx = await tryCreateATAIx(
            this.connection, this.wallet.publicKey, proceedsAta, VAULT, NativeMint.address, TOKEN_2022_PROGRAM_ID
        );
        ixs.push(...proceedsIx);

        const renewIx = await this.program.methods
            .tokenRenew({ renewTerms: 1 })
            .accounts({
                mint,
                tokenMetadata: this.getTokenBurgerMetadata(mint),
                mintPayment: NativeMint.address,
                proceedsTokenAccount: proceedsAta,
                payerTokenAccount: payerAta,
                payer: this.wallet.publicKey,
                updateAuthority: this.getProgramDelegate(),
                token22Program: TOKEN_2022_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID
            })
            .instruction();
        ixs.push(renewIx);

        const id = await sendAndConfirmRawTransaction(
            this.connection, new Transaction().add(...ixs), this.wallet.publicKey, this.wallet, [...wrapSolTxn.signers]
        );

        return id;
    }

    getProgramDelegate(): PublicKey {
        const [programDelegate] = PublicKey.findProgramAddressSync(
            [Buffer.from("BURGER_DELEGATE")],
            this.program.programId
        );
        return programDelegate;
    }

    getTokenBurgerMetadata(mint: PublicKey): PublicKey {
        const [metadata] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("burgermetadata"),
                mint.toBuffer()
            ],
            this.program.programId
        );
        return metadata;
    }

}