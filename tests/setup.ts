import {loadOrGenerateKeypair, savePublicKeyToFile, stringify2} from "./utils/helpers";
import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey,
} from "@solana/web3.js";
import {
    ExtensionType,
    createInitializeMintInstruction,
    createInitializePermanentDelegateInstruction,
    mintTo,
    getMintLen,
    TOKEN_2022_PROGRAM_ID,
    getOrCreateAssociatedTokenAccount,
    createBurnInstruction,
    createInitializeMintCloseAuthorityInstruction,
    createCloseAccountInstruction,
} from "@solana/spl-token";
import {metadataInstruction} from "./instructions/createInitializeTokenMetadataInstruction";
import {createInitializeMetadataPointerInstruction} from "./instructions/createInitializeMetadataPointerInstruction";
import { Token22Layout } from "./state/token22";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");


async function setup() {
    const payer = loadOrGenerateKeypair("payer");
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");
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



        // instruction 25
        createInitializeMintCloseAuthorityInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        // instruction 35
        createInitializePermanentDelegateInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),

        // instruction 39
        // metadatapointer should happen after Account creation, before mint initialization
        // Error because account sizing is wrong. Proper space has been allocated to the above two, but not the metadatapointer
        // If I put this as the first ix, it succeeds

        // So there is a difference between the span/sizing of an instruction and the config/account/state size.
        // so basically had to account for how it does the computations on sizing. All I needed was two pubkey sizing (32*2=64)
        // in addition to 2 + 2 for the default computational aspects SIZE+LENGTH
        createInitializeMetadataPointerInstruction(mint, permanentDelegate.publicKey, mint, TOKEN_2022_PROGRAM_ID),


        createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID),

        // Need to transfer to mint before can init metadata
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


    const transaction = new Transaction().add(
        createBurnInstruction(account.address, mint, permanentDelegate.publicKey, 1, [], TOKEN_2022_PROGRAM_ID),
        createCloseAccountInstruction(mint, payer.publicKey, permanentDelegate.publicKey, [], TOKEN_2022_PROGRAM_ID)
    );


    const tx = await sendAndConfirmTransaction(connection, transaction, [permanentDelegate]);
    console.log("tx", tx);

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
    const info = await connection.getAccountInfo(new PublicKey("8MBcTD24nCZeN3f73RNFCGW5HcD4C3y62VwjvLz8xpjr"));
    const decoded = Token22Layout.decode(info.data);
    // no need for decoding
    // const decoded = AccountLayout.decode(info.data.slice(8));
    console.log("decoded", stringify2(decoded));
}


try {
    accountInfo();
    // setup();
    // mint();

    // burn();
    // test();
} catch (e) {
    console.log("err", e);
}