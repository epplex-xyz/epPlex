import {loadOrGenerateKeypair, savePublicKeyToFile} from "./utils/helpers";
import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey,
    TransactionInstruction,
    AccountMeta
} from "@solana/web3.js";

import {
    ExtensionType,
    createInitializeMintInstruction,
    createInitializePermanentDelegateInstruction,
    mintTo,
    createAccount,
    getMintLen,
    transferChecked,
    TOKEN_2022_PROGRAM_ID,
    getOrCreateAssociatedTokenAccount,
    createBurnInstruction,
    burnInstructionData,
    TokenInstruction,
    createInitializeMintCloseAuthorityInstruction,
    createCloseAccountInstruction
} from "@solana/spl-token";
import * as borsh from "@coral-xyz/borsh";
import {metadataInstruction} from "./createInitializeTokenMetadataInstruction";
import {createInitializeMetadataPointerInstruction} from "./createInitializeMetadataPointerInstruction";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");

// Old mint https://solscan.io/token/3s792R18rLLvrGmFYk373jVSML7xh6SvsW5ZiXTxTk3Y?cluster=devnet, only has authority field

const layout = borsh.struct([
    borsh.publicKey("updateAuthority"),
    borsh.publicKey("mint"),
    borsh.array(borsh.u8(), 11, "name"),
    borsh.array(borsh.u8(), 4, "symbol"),
    borsh.array(borsh.u8(), 20, "uri"),
]);


const TOKEN_METADATA_SIZE = layout.span + 500;


async function setup() {

    // Collection auth and treeCreator
    const payer = loadOrGenerateKeypair("payer");
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");
    // const permanentDelegate = new PublicKey("2N6aJDX1TNs6RKkPsuufbAe4JjRAZPs1iLPcEUL4DX4z");

    // const airdropSignature = await connection.requestAirdrop(payer.publicKey, 2 * LAMPORTS_PER_SOL);
    // await connection.confirmTransaction({ signature: airdropSignature, ...(await connection.getLatestBlockhash()) });


    const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
    // const mintLen = getMintLen(extensions) + TOKEN_METADATA_SIZE;
    const mintLen = getMintLen(extensions) + (64 + 2 + 2);
    // console.log("length", getMintLen(extensions));
    // console.log("length", mintLen);
    const decimals = 0;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);


    const mintTransaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint,
            space: mintLen,
            lamports: mintLamports,
            programId: TOKEN_2022_PROGRAM_ID,
        }),


        createInitializeMintCloseAuthorityInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        createInitializePermanentDelegateInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        // TODO
        // metadatapointer should happen after Account creation, before mint initialization
        // Error because account sizing is wrong. Proper space has been allocated to the above two, but not the metadatapointer
        // If I put this as the first ix, it succeeds
        createInitializeMetadataPointerInstruction(mint, permanentDelegate.publicKey, mint, TOKEN_2022_PROGRAM_ID),

        createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID),

        // These two are tied together
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: mint,
            lamports: BigInt(1000000),
        }),
        metadataInstruction(mint, permanentDelegate.publicKey, mint, mintAuthority.publicKey),
    );
    const txId = await sendAndConfirmTransaction(connection, mintTransaction, [payer, mintKeypair, mintAuthority], {skipPreflight: true});
    console.log("tx", txId);

    savePublicKeyToFile("mintPubkey", mint);
}

async function mint() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");

    // Get the token account of the toWallet address, and if it does not exist, create it
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey,
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("token", fromTokenAccount.address.toString());

    // Mint 1 new token to the "fromTokenAccount" account we just created
    let signature = await mintTo(
        connection,
        payer,
        mint,
        fromTokenAccount.address,
        mintAuthority,
        1,
        [],
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("tx", signature);
}


async function burn() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
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


    const keys: AccountMeta[] = [
        { pubkey: account.address, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: true },
        { pubkey: permanentDelegate.publicKey, isSigner: true, isWritable: true }
    ];

    const data = Buffer.alloc(burnInstructionData.span);
    burnInstructionData.encode(
        {
            instruction: TokenInstruction.Burn,
            amount: BigInt(1),
        },
        data
    );

    const ix = new TransactionInstruction({ keys, programId: TOKEN_2022_PROGRAM_ID, data });

    const transaction = new Transaction().add(
        createBurnInstruction(account.address, mint, permanentDelegate.publicKey, 1, [], TOKEN_2022_PROGRAM_ID),
        createCloseAccountInstruction(mint, payer.publicKey, permanentDelegate.publicKey, [], TOKEN_2022_PROGRAM_ID)
    );

    try {
        const tx = await sendAndConfirmTransaction(connection, transaction, [permanentDelegate]);
        console.log("tx", tx);
    } catch (e) {
        console.log("err", e);
    }
}

async function test() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");


    const transaction = new Transaction().add(
        metadataInstruction(mint, permanentDelegate.publicKey, mint, mintAuthority.publicKey)
    );

    try {
        const tx = await sendAndConfirmTransaction(connection, transaction, [payer, mintAuthority], {skipPreflight: true});
        console.log("tx", tx);
    } catch (e) {
        console.log("err", e);
    }

}

async function accountInfo() {
    const res = await connection.getAccountInfo(new PublicKey("bbs4CMz9JL3JBW7wH7wq4q3sbBRV3PVAK5U7iKgFKFN"));
    console.log("Res",res);

}
// accountInfo();
setup();
// mint();
// test();
// burn();
// console.log("size", TOKEN_METADATA_SIZE);