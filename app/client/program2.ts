import {
    Connection,
    Keypair, ParsedAccountData,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram, SYSVAR_RENT_PUBKEY, Transaction,
} from "@solana/web3.js";
import { createProgram, EphemeralityProgram } from "./types/programTypes";
import {AnchorProvider, Wallet} from "@coral-xyz/anchor";
import { mintToIx, sendAndConfirmRawTransaction, tryCreateATAIx2 } from "../utils/solana";
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
        // Probably this needs to be just get associated token account
        // const account = await getOrCreateAssociatedTokenAccount(
        //     this.connection,
        //     payer,
        //     mint,
        //     payer.publicKey,
        //     undefined,
        //     undefined,
        //     undefined,
        //     TOKEN_2022_PROGRAM_ID
        // );
        //
        const info = await this.connection.getAccountInfo(mint);
        if (info === null){
            throw Error("Mint does not exist");
        }
        // const mintKey = new PublicKey("FfWP2mXizKnHZLsG3mTDFC2vWoZFfZTQi1Rpvm2nQTgM");
        const largestAccounts = await this.connection.getTokenLargestAccounts(mint);
        const largestAccountInfo = await this.connection.getParsedAccountInfo(
            largestAccounts.value[0].address  //first element is the largest account, assumed with 1
        );
        if (largestAccountInfo.value === null){
            throw Error("Largest account does not exist");
        }
        const owner = (largestAccountInfo.value.data as ParsedAccountData).parsed.info.owner;
        console.log(" owner23 sds " ,(largestAccountInfo.value.data as ParsedAccountData).parsed.info.owner);
        // console.log("owner", info,/**/ JSON.stringify(info));
        const ata = getAssociatedTokenAddressSync(
            mint,
            new PublicKey(owner),
            undefined,
            TOKEN_2022_PROGRAM_ID
        );
        // const a = await this.connection.getAccountInfo(ata);
        console.log("info",ata.toString());

        // const resDestination = await tryCreateATAIx2(this.connection, this.wallet.payer, , token);
        // if (resDestination === undefined) {
        //     throw new Error("try create destination ATA failed");
        // } else if (Array.isArray(resDestination)) {
        //     const [ix, ata] = resDestination;
        //     destinationAta = ata;
        //     ixs.push(ix);
        // } else {
        //     destinationAta = resDestination;
        // }

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