import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { BurgerProgram } from "../app/client/burgerProgram";
import {Program2} from "../app/client/program2";
import { BN, Wallet } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    getTokenMetadata,
    TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import * as pda from './pda';
import { buildNFTTransferTx, getToken22 } from "../app/utils/token2022";
import { createBurnAndCloseIx, createTokenCloseAndBurnIx } from "../script/instructions/generic";
import { loadKeypairFromFile, printConsoleSeparator } from "../script/utils/helpers";

// This works
// import dotenv from "dotenv";
// import path from "path";
// dotenv.config({path: path.resolve(__dirname, "../.env.local")})
// console.log("prces", process.env.MINT_POOL_KEYPAIR)

const secretKeypair = loadKeypairFromFile("/home/fzzyyti/.config/solana/test.json")
const mintPool = loadKeypairFromFile("/home/fzzyyti/.config/solana/mint.json")

describe('Environment setup', () => {
    const tempProvider = anchor.AnchorProvider.env();
    anchor.setProvider(tempProvider);

    console.log("secret", secretKeypair.publicKey.toString());
    const provider = new anchor.AnchorProvider(
        tempProvider.connection,
        new Wallet(secretKeypair),
        {skipPreflight: true}
    )
    console.log("provider", provider.wallet.publicKey.toString())
    anchor.setProvider(provider);
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);
    const coreProgram = new Program2(provider.wallet, provider.connection);

    console.log("mint", mintPool.publicKey.toString());

    const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
    console.log("destroy", destroyTimestamp);
    const mint = Keypair.generate();

    it("Create burger delegate ", async() => {
        await burgerProgram.createProgramDelegate();
    })

    it('Mint token', async () => {
        coreProgram.createGlobalCollectionConfig();
        await new Promise((resolve) => setTimeout(resolve, 1000));
        const globalCollectionAddress = coreProgram.getGlobalCollectionConfigAddress();
        console.log("globalCollectionAddress", globalCollectionAddress.toString());
        const globalCollectionData = await coreProgram.program.account.globalCollectionConfig.fetch(
            coreProgram.getGlobalCollectionConfigAddress());
        const [collectionConfigAddress] = PublicKey.findProgramAddressSync(
            [Buffer.from("CONFIG"),
                globalCollectionData.collectionCounter.toArrayLike(Buffer, "le", 8)],
            coreProgram.program.programId
        )

        await coreProgram.createCollection(collectionConfigAddress, burgerProgram.getProgramDelegate());

        const collection = await coreProgram.program.account.collectionConfig.fetch(collectionConfigAddress);
        const tx = await burgerProgram.createWhitelistMintTx(
            destroyTimestamp,
            collectionConfigAddress,
            globalCollectionData.collectionCounter,
            mint,
        )
        await sendAndConfirmRawTransaction(
            provider.connection,
            tx,
            provider.publicKey,
            provider.wallet,
            [mint]
        );
        console.log("confirm mint")
        const metadata = await getTokenMetadata(
            provider.connection,
            mint.publicKey
        )
        console.log("metadata", metadata);
    });

    it('Transfer token', async () => {
        const tx = await buildNFTTransferTx({
            connection: provider.connection,
            mint: mint.publicKey,
            source: provider.wallet.publicKey,
            destination: mintPool.publicKey,
            payer: secretKeypair.publicKey,
        })

        await sendAndConfirmRawTransaction(
                provider.connection,
                tx,
                secretKeypair.publicKey,
                undefined,
                [secretKeypair]
            );
    });

    it('Renew token', async () => {
        await burgerProgram.renewToken(mint.publicKey)
    });

    // TODO uncomment if you want to burn your tokens
    // it('Burn tokens', async () => {
    //      const burgerDelegate = burgerProgram.getProgramDelegate();
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
