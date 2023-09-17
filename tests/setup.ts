import {loadOrGenerateKeypair, savePublicKeyToFile, stringify2} from "./utils/helpers";
import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey, Keypair, SYSVAR_RENT_PUBKEY,
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
import {createMetadataInstruction, updateMetadataInstruction} from "./instructions/tokenMetadataInstructions";
import {createInitializeMetadataPointerInstruction} from "./instructions/createInitializeMetadataPointerInstruction";
import { Token22Layout } from "./state/token22";
import { Program } from "./program";
import { CONFIRM_OPTIONS } from "../client/constants";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");
const METADATAPOINTER_SIZE = 64 + 2 + 2;


async function setup() {
    const payer = loadOrGenerateKeypair("payer");
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");
    // const airdropSignature = await connection.requestAirdrop(payer.publicKey, 2 * LAMPORTS_PER_SOL);
    // await connection.confirmTransaction({ signature: airdropSignature, ...(await connection.getLatestBlockhash()) });

    const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
    const mintLen = getMintLen(extensions) + METADATAPOINTER_SIZE;
    const decimals = 0;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const mintTransaction = new Transaction().add(
        // Creates empty account with space that is ready to be allocated
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
        createInitializeMetadataPointerInstruction(mint, permanentDelegate.publicKey, mint, TOKEN_2022_PROGRAM_ID),
        // instruction 0, actually populates the allocated account from the first instruction
        createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID),

        // Need to transfer to mint before can init metadata
        // TODO: not the best hardcode
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: mint,
            lamports: BigInt(10000000),
        }),
        // Custom instruction
        createMetadataInstruction(mint, permanentDelegate.publicKey, mint, mintAuthority.publicKey),
        updateMetadataInstruction(mint, permanentDelegate.publicKey)
    );
    const txId = await sendAndConfirmTransaction(connection, mintTransaction, [payer, mintKeypair, mintAuthority, permanentDelegate], {skipPreflight: true});
    console.log("tx", txId);

    savePublicKeyToFile("mintPubkey", mint);
    savePublicKeyToFile("mintAuth", mintAuthority.publicKey);
    savePublicKeyToFile("permDelegate", permanentDelegate.publicKey);
    savePublicKeyToFile("payer", payer.publicKey);
}

async function mint() {
    const payer = loadOrGenerateKeypair("payer");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    // const mintAuthority = loadOrGenerateKeypair("mintAuth");

    // Get the token account of the toWallet address, and if it does not exist, create it
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


async function burn() {
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
async function accountInfo() {
    const info = await connection.getAccountInfo(new PublicKey("6DoTJakcvoKwXougVGmwGkPWuB2pGLGXGNhwxTx46Rq"));
    const decoded = Token22Layout.decode(info.data);
    // no need for decoding
    // const decoded = AccountLayout.decode(info.data.slice(8));
    console.log("decoded", stringify2(decoded));
}

// const SIGNER = Keypair.fromSecretKey(new Uint8Array(JSON.parse(process.env.SIGNER_KEYPAIR)));

async function test() {
    const payer = loadOrGenerateKeypair("payer");
    const program = new Program(payer, connection);
    const programDelegate = program.getProgramDelegate();

    const initDelegateIx = await program.program.methods
        .programDelegateCreate({})
        .accounts({
            programDelegate,
            payer: payer.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .instruction();

    const mint = loadOrGenerateKeypair("mint");
    console.log("mint", mint.publicKey.toString());

    const tokenCreateIx = await program.program.methods
        .tokenCreate({})
        .accounts({
            mint: mint.publicKey,
            programDelegate: programDelegate,
            payer: payer.publicKey,
            systemProgram: SystemProgram.programId,
            token22Program: TOKEN_2022_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .instruction();

    const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
    const mintLen = getMintLen(extensions) + METADATAPOINTER_SIZE;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(...[
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint.publicKey,
            space: mintLen,
            lamports: mintLamports,
            programId: TOKEN_2022_PROGRAM_ID,
        }),
        initDelegateIx,
        tokenCreateIx
    ]);

    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, mint], CONFIRM_OPTIONS);
    console.log("tx", tx);
}

async function test2() {
    const payer = loadOrGenerateKeypair("payer");
    const program = new Program(payer, connection);
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const programDelegate = program.getProgramDelegate();

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

    const tokenBurnTx = await program.program.methods
        .tokenBurn({})
        .accounts({
            mint: mint,
            programDelegate: programDelegate,
            tokenAccount: account.address,
            token22Program: TOKEN_2022_PROGRAM_ID,
        })
        .transaction();

    const tx = await sendAndConfirmTransaction(connection, tokenBurnTx, [payer], CONFIRM_OPTIONS);
    console.log("tx", tx);
}

async function test3() {
    const payer = loadOrGenerateKeypair("payer");
    const program = new Program(payer, connection);
    const programDelegate = program.getProgramDelegate();

    const tokenBurnTx = await program.program.methods
        .programDelegateClose({})
        .accounts({
            programDelegate: programDelegate,
            payer: payer.publicKey,
        })
        .transaction();

    const tx = await sendAndConfirmTransaction(connection, tokenBurnTx, [payer], CONFIRM_OPTIONS);
    console.log("tx", tx);
}
async function main() {
    try {
        await test();
        // await accountInfo();
        // await setup();
        // await mint();
        // await burn();
        // await test();
    } catch (e) {
        console.log("err", e);
    }
}

main();