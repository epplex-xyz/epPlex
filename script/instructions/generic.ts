import { loadOrGenerateKeypair } from "../utils/helpers";
import {
    createBurnInstruction,
    createCloseAccountInstruction,
    getOrCreateAssociatedTokenAccount, mintTo,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import {
    Connection,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
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

export async function createBurnAndCloseIx(connection: Connection, payer: Keypair, mint: PublicKey, closeAuth: PublicKey) {
    // WHy do we need to create this?
    // We need to close the token account as well

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

    const ixs = [
        //1. Burns the token amount from the Token account
        createBurnInstruction(account.address, mint, payer.publicKey, 1, [], TOKEN_2022_PROGRAM_ID),

        //2. Actually closes the mint account
        createCloseAccountInstruction(mint, payer.publicKey, closeAuth, [], TOKEN_2022_PROGRAM_ID)

        // Obviously can't burn the token account since it owned by the owner
        // Should also try to close the token account
    ];

    return ixs
}

export async function createTokenCloseAndBurnIx(connection: Connection, payer: Keypair, mint: PublicKey) {
    // WHy do we need to create this?
    // We need to close the token account as well

    // Get the token account of the toWallet address, and if it does not exist, create it
    // TODO This should probably not call create
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

    const ixs: TransactionInstruction[] = [];

    if (Number(account.amount) === 1){
        ixs.push(
            createBurnInstruction(account.address, mint, payer.publicKey, 1, [], TOKEN_2022_PROGRAM_ID)
        )
    }

    ixs.push(
        createCloseAccountInstruction(account.address, payer.publicKey, payer.publicKey, [], TOKEN_2022_PROGRAM_ID)
    )

    return ixs
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

