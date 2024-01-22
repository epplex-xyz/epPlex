import { Connection } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { BurgerProgram } from "../app/client/burgerProgram";
import {Program2} from "../app/client/program2";
import { BN } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import * as pda from './pda';

describe('Environment setup', () => {
    // const {
    //     coreProgram,
    //     burgerProgram,
    //     metadataProgram,
    //     connection,
    //     wallet
    // } = testPrelude();

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);
    //
    // const mainProgram = new Program2(wallet, connection);
    // const burgerProgramDelegate = pda.burgerProgramDelegate(burgerProgram.programId);
    // const mint = Keypair.generate();
    // const payer = wallet.publicKey
    // const ata = getAssociatedTokenAddressSync(
    //     mint.publicKey,
    //     payer,
    //     undefined,
    //     TOKEN_2022_PROGRAM_ID,
    //     ASSOCIATED_TOKEN_PROGRAM_ID
    // );

    // const tm = tokenMetadata(mint.publicKey, metadataProgram.programId)


    it("Create burger delegate ", async() => {
        await burgerProgram.createProgramDelegate();
    })

    it('Mint tokens', async () => {
        // async function burnTokens() {
        //
        //     // 1706020091
        //     // const payer = loadOrGenerateKeypair("payer");
        //     // const mintKeypair = loadOrGenerateKeypair("mint");
        //     // const program = new Program(payer, connection);
        //
        //
        //     // await program.createToken(mintKeypair, payer);
        //     // await mint(connection, mintKeypair.publicKey, payer);
        //     // await program.burnToken(mintKeypair.publicKey, payer);
        // }
    });

    it('Burn tokens', async () => {
        // async function burnTokens() {
        //
        //     // 1706020091
        //     // const payer = loadOrGenerateKeypair("payer");
        //     // const mintKeypair = loadOrGenerateKeypair("mint");
        //     // const program = new Program(payer, connection);
        //
        //
        //     // await program.createToken(mintKeypair, payer);
        //     // await mint(connection, mintKeypair.publicKey, payer);
        //     // await program.burnToken(mintKeypair.publicKey, payer);
        // }
    });
});
