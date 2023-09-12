import {loadOrGenerateKeypair, savePublicKeyToFile} from "./utils/helpers";
import {Connection, Keypair, LAMPORTS_PER_SOL, Transaction, SystemProgram, sendAndConfirmTransaction, PublicKey,TransactionInstruction, AccountMeta} from "@solana/web3.js";
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
    createInitializeMintCloseAuthorityInstruction
} from "@solana/spl-token";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");

async function setup() {

    // Collection auth and treeCreator
    const payer = loadOrGenerateKeypair("payer");

    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");
    // const permanentDelegate = new PublicKey("2N6aJDX1TNs6RKkPsuufbAe4JjRAZPs1iLPcEUL4DX4z");


    const extensions = [ExtensionType.PermanentDelegate];
    const mintLen = getMintLen(extensions);
    const decimals = 0;

    const airdropSignature = await connection.requestAirdrop(payer.publicKey, 2 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction({ signature: airdropSignature, ...(await connection.getLatestBlockhash()) });

    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);
    const mintTransaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint,
            space: mintLen,
            lamports: mintLamports,
            programId: TOKEN_2022_PROGRAM_ID,
        }),
        // createInitializeMintCloseAuthorityInstruction(mint,permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        createInitializePermanentDelegateInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID)
    );
    const txId = await sendAndConfirmTransaction(connection, mintTransaction, [payer, mintKeypair], undefined);
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

    // createBurnInstruction(account, mint, ownerPublicKey, amount, multiSigners, programId)
    // const keys: AccountMeta[] = [
    //     { pubkey: account.address, isSigner: false, isWritable: true },
    //     { pubkey: mint, isSigner: false, isWritable: true },
    //     { pubkey: permanentDelegate.publicKey, isSigner: true, isWritable: true }
    // ];

    // const data = Buffer.alloc(burnInstructionData.span);
    // burnInstructionData.encode(
    //     {
    //         instruction: TokenInstruction.Burn,
    //         amount: BigInt(1),
    //     },
    //     data
    // );

    // const ix = new TransactionInstruction({ keys, programId: TOKEN_2022_PROGRAM_ID, data });

    const transaction = new Transaction().add(
        createBurnInstruction(account.address, mint, permanentDelegate.publicKey, 1, [], TOKEN_2022_PROGRAM_ID)
    );

    try {
        const tx = await sendAndConfirmTransaction(connection, transaction, [permanentDelegate], {skipPreflight: true});
        console.log("tx", tx);
    } catch (e) {
        console.log("err", e);
    }



}

burn();