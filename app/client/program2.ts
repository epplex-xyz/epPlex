import { Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { createCoreProgram, EpplexCoreProgram } from "./types/programTypes";
import { AnchorProvider, BN, Wallet } from "@coral-xyz/anchor";
import { getMintOwner, sendAndConfirmRawTransaction } from "../utils/solana";
import { CONFIRM_OPTIONS } from "./constants";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { AnchorWallet } from "@solana/wallet-adapter-react";

export class Program2 {
    connection: Connection;
    program: EpplexCoreProgram;
    wallet: Wallet;
    constructor(
        wallet: AnchorWallet,
        connection: Connection,
    ) {
        const provider = new AnchorProvider(connection, wallet, CONFIRM_OPTIONS);
        this.program = createCoreProgram(provider);
        this.connection = connection;
        this.wallet = (this.program.provider as AnchorProvider).wallet as Wallet;
    }
    // async createToken(
    //     destroyTimestampOffset: number = 60 * 5,
    //     name: string = "Ephemeral burger",
    //     symbol: string = "EP",
    //     uri: string = "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo",
    // ) {
    //     const programDelegate = this.getProgramDelegate();
    //     const payer = this.wallet.publicKey;
    //     const mint = Keypair.generate();
    //     const ata = getAssociatedTokenAddressSync(
    //         mint.publicKey,
    //         payer,
    //         undefined,
    //         TOKEN_2022_PROGRAM_ID,
    //         ASSOCIATED_TOKEN_PROGRAM_ID
    //     );
    //     const tm = this.getTokenMetadata(mint.publicKey);
    //
    //     // const tokenCreateTx = await this.program.methods
    //     //     .tokenMint({
    //     //         destroyTimestampOffset: new BN(destroyTimestampOffset),
    //     //         name: name,
    //     //         symbol: symbol,
    //     //         uri: uri,
    //     //     })
    //     //     .accounts({
    //     //         mint: mint.publicKey,
    //     //         ata,
    //     //         tokenMetadata: tm,
    //     //         programDelegate: programDelegate,
    //     //         payer: payer,
    //     //         systemProgram: SystemProgram.programId,
    //     //         token22Program: TOKEN_2022_PROGRAM_ID,
    //     //         rent: SYSVAR_RENT_PUBKEY,
    //     //         associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
    //     //     })
    //     //     .transaction();
    //     //
    //     // let id;
    //     // try {
    //     //     id = await sendAndConfirmRawTransaction(
    //     //         this.connection,
    //     //         tokenCreateTx,
    //     //         payer,
    //     //         this.wallet,
    //     //         [mint]
    //     //     );
    //     //     console.log("tx", id);
    //     // } catch (e) {
    //     //     console.log("Failed to send tx", e);
    //     // }
    //     // return id;
    // }
    //
    // async burnToken(
    //     mint: PublicKey,
    // ) {
    //     const programDelegate = this.getProgramDelegate();
    //     const mintOwner = await getMintOwner(this.connection, mint);
    //     const ata = getAssociatedTokenAddressSync(
    //         mint,
    //         mintOwner,
    //         undefined,
    //         TOKEN_2022_PROGRAM_ID
    //     );
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
    //
    // async createProgramDelegate() {
    //     const programDelegate = this.getProgramDelegate();
    //
    //     const tx = await this.program.methods
    //         .programDelegateCreate({})
    //         .accounts({
    //             programDelegate,
    //             payer: this.wallet.publicKey,
    //             systemProgram: SystemProgram.programId,
    //         })
    //         .transaction();
    //
    //     const id = await sendAndConfirmRawTransaction(
    //         this.connection, tx, this.wallet.publicKey, this.wallet, []
    //     );
    //     console.log("tx", id);
    //
    //     return id;
    // }
    //
    // async renewToken() {
    //     const mint = new PublicKey("DRa4aV8SMbcM9g9aDiiWnHgmwNcNNcxvTHh1Bfg1z1zJ");
    //     const ata = getAssociatedTokenAddressSync(
    //         mint,
    //         this.wallet.publicKey,
    //         undefined,
    //         TOKEN_2022_PROGRAM_ID
    //     );
    //     const programDelegate = this.getProgramDelegate();
    //
    //     const tx = await this.program.methods
    //         .tokenRenew({renewTerms: 1})
    //         .accounts({
    //             mint,
    //             tokenAccount: ata,
    //             programDelegate,
    //             authority: this.wallet.publicKey,
    //             token22Program: TOKEN_2022_PROGRAM_ID,
    //         })
    //         .transaction();
    //
    //     const id = await sendAndConfirmRawTransaction(
    //         this.connection, tx, this.wallet.publicKey, this.wallet, []
    //     );
    //     console.log("tx", id);
    //
    //     return id;
    // }
    //
    //
    // getProgramDelegate(): PublicKey {
    //     const [programDelegate] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("PROGRAM_DELEGATE")],
    //         this.program.programId
    //     );
    //     return programDelegate;
    // }
    //
   public getGlobalCollectionConfigAddress(): PublicKey {
        const [globalCollectionConfig] = PublicKey.findProgramAddressSync(
            [Buffer.from("GLOBAL_COLLECTION")],
            this.program.programId
        );
        return globalCollectionConfig;
    }

    async createGlobalCollectionConfig() {
        const globalCollectionConfig = this.getGlobalCollectionConfigAddress();
        const tx = await this.program.methods
            .globalCollectionConfigCreate()
            .accounts({
                globalCollectionConfig,
                payer: this.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .transaction();

        const id = await sendAndConfirmRawTransaction(
            this.connection, tx, this.wallet.publicKey, this.wallet, []
        );
        console.log("Create global config tx", id);

        return id;
    }

    async createCollection(collectionConfigAddress: PublicKey, authority: PublicKey) {

        const builder = this.program.methods
            .collectionCreate({
                renewalPrice: new BN(1000000000),
                mintPrice: new BN(1000000000),
                standardDuration: 10000,
                gracePeriod: new BN(10000),
                treasury: this.wallet.publicKey,
                collectionSize: 100,
                collectionName: "Epplex",
                collectionSymbol: "EPX",
                authority,
            })
            .accounts({
                collectionConfig: collectionConfigAddress,
                globalCollectionConfig: this.getGlobalCollectionConfigAddress(),
                payer: this.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            });

        const tx = await builder.transaction();

        const id = await sendAndConfirmRawTransaction(
            this.connection, tx, this.wallet.publicKey, this.wallet, []
        );
        console.log("creating collection tx", id);

        return id;
    }
    //
    // getTokenMetadata(mint: PublicKey): PublicKey {
    //     const [programDelegate] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("metadata"), this.program.programId.toBuffer(), mint.toBuffer()],
    //         this.program.programId
    //     );
    //     return programDelegate;
    // }
    //
    // static staticGetTokenMetadata(mint: PublicKey): PublicKey {
    //     // TODO dont hardcode this
    //     const pid = new PublicKey("epPgfrTRUdijJdkjn6EYBNsPrf8YSV7JeUGGhWSwkex");
    //     const [programDelegate] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("metadata"), pid.toBuffer(), mint.toBuffer()],
    //         pid
    //     );
    //     return programDelegate;
    // }

}