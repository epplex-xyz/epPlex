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
import { Program } from "../app/client/program"
import {mint} from "./instructions/generic";
import * as anchor from "@coral-xyz/anchor";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");
const METADATAPOINTER_SIZE = 64 + 2 + 2;


async function mintTokens() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

}

async function burnTokens() {
    const payer = loadOrGenerateKeypair("payer");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const program = new Program(payer, connection);


    // await program.createToken(mintKeypair, payer);
    await mint(connection, mintKeypair.publicKey, payer);
    // await program.burnToken(mintKeypair.publicKey, payer);
}

async function burnTokens() {
    try {
        // await setup();
        await test();
    } catch (e) {
        console.log("err", e);
    }
}
// main();