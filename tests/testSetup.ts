import { Connection, Transaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { BurgerProgram } from "../app/client/burgerProgram";
import {Program2} from "../app/client/program2";
import { BN, Wallet } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import * as pda from './pda';
import { getToken22 } from "../app/utils/token2022";
import { createBurnAndCloseIx, createTokenCloseAndBurnIx } from "../script/instructions/generic";
import { loadKeypairFromFile } from "../script/utils/helpers";

const secretKeypair = loadKeypairFromFile("/Users/Mac/.config/solana/test.json")

describe('Environment setup', () => {
    const tempProvider = anchor.AnchorProvider.env();
    anchor.setProvider(tempProvider);

    const provider = new anchor.AnchorProvider(
        tempProvider.connection,
        new Wallet(secretKeypair),
        {skipPreflight: true}
    )
    anchor.setProvider(provider);
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);

    const burgerDelegate = burgerProgram.getProgramDelegate();
    const destroyTimestamp: string = "1706020091"

    // it("Create burger delegate ", async() => {
    //     await burgerProgram.createProgramDelegate();
    // })

    it('Mint tokens', async () => {
        await burgerProgram.createWhitelistMint(destroyTimestamp)
    });

    // TODO uncomment if you want to burn your tokens
    // it('Burn tokens', async () => {
    //     const allTokens = await getToken22(
    //         provider.connection,
    //         provider.publicKey
    //     )
    //
    //     console.log("Total tokens", allTokens.length);
    //     // Close one by one
    //     for (const mint of allTokens) {
    //         console.log("Closing mint", mint.toString());
    //         const ixs = await createTokenCloseAndBurnIx(
    //             provider.connection,
    //             secretKeypair,
    //             mint,
    //         )
    //
    //         await sendAndConfirmRawTransaction(
    //             provider.connection,
    //             new Transaction().add(...ixs),
    //             secretKeypair.publicKey,
    //             undefined,
    //             [secretKeypair]
    //         )
    //     }
    // });
});
