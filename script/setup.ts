import {loadOrGenerateKeypair, savePublicKeyToFile} from "./utils/helpers";
import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction, PublicKey,
} from "@solana/web3.js";
import {
    ExtensionType,
    createInitializeMintInstruction,
    createInitializePermanentDelegateInstruction,
    getMintLen,
    TOKEN_2022_PROGRAM_ID,
    createInitializeMintCloseAuthorityInstruction,
} from "@solana/spl-token";
import {createMetadataInstruction, updateMetadataInstruction} from "./instructions/tokenMetadataInstructions";
import {createInitializeMetadataPointerInstruction} from "./instructions/createInitializeMetadataPointerInstruction";
import {mint} from "./instructions/generic";

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

        // Need to transfer to mint address before metadata can be initialised
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

async function test() {
    const payer = loadOrGenerateKeypair("payer");
    const mintKeypair = loadOrGenerateKeypair("mint");
    // const program = new Program(payer, connection);


    // await program.createToken(mintKeypair, payer);
    await mint(connection, mintKeypair.publicKey, payer);
    // await program.burnToken(mintKeypair.publicKey, payer);
}

function byteToPubkeyString() {
    const inputString = "234 86 246 193 255 234 210 52 77 211 118 42 111 252 130 84 81 176 232 153 129 251 194 18 168 86 187 130 50 172 81 43";

    // Split the input string into an array of strings representing individual numbers
    const numStrings = inputString.split(" ");

    // Convert each string to a number
    const numArray = numStrings.map((numString) => parseInt(numString, 10));
    const pub = new PublicKey(new Uint8Array(numArray));
    console.log(pub.toString());
}

async function main() {
    try {
        // await setup();
        await test();
    } catch (e) {
        console.log("err", e);
    }
}
byteToPubkeyString()
// main();