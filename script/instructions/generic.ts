import { loadOrGenerateKeypair } from "../utils/helpers";
import {
    createBurnInstruction,
    createCloseAccountInstruction,
    getOrCreateAssociatedTokenAccount, mintTo,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { CONFIRM_OPTIONS } from "../../app/client/constants";

async function burn(connection: Connection) {
    const payer = loadOrGenerateKeypair("payer");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");

    // Get the token account of the toWallet address, and if it does not exist, create it
    const account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey,
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );


    const transaction = new Transaction().add(
        // Burns the token amount from the Token account
        // Obviously can't burn the token account since it owned by the owner
        createBurnInstruction(account.address, mint, permanentDelegate.publicKey, 1, [], TOKEN_2022_PROGRAM_ID),
        // Actually closes the mint account
        createCloseAccountInstruction(mint, payer.publicKey, permanentDelegate.publicKey, [], TOKEN_2022_PROGRAM_ID)
    );

    const tx = await sendAndConfirmTransaction(connection, transaction, [permanentDelegate], CONFIRM_OPTIONS);
    console.log("tx", tx);

}

export async function mint(connection: Connection, mint: PublicKey, payer: Keypair) {
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey, // owner
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    const signature = await mintTo(
        connection,
        payer,
        mint,
        fromTokenAccount.address,
        payer,
        1,
        [],
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("tx", signature);
}

