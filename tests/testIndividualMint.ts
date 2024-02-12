import {PublicKey} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {BN, Wallet} from "@coral-xyz/anchor";
import {BurgerProgram} from "../app/client/burgerProgram";
import {CoreProgram} from "../app/client/coreProgram";
import {getTokenMetadata} from "@solana/spl-token";
import {sendAndConfirmRawTransaction} from "../app/utils/solana";
import {buildNFTTransferTx} from "../app/utils/token2022";
import {loadKeypairFromFile} from "../script/utils/helpers";

// This works
// import dotenv from "dotenv";
// import path from "path";
// dotenv.config({path: path.resolve(__dirname, "../.env.local")})
// console.log("prces", process.env.MINT_POOL_KEYPAIR)

const payer = loadKeypairFromFile("../target/deploy/epplex_PAYER_ADMIN.json")
const destination = new PublicKey("2N6aJDX1TNs6RKkPsuufbAe4JjRAZPs1iLPcEUL4DX4z")

describe('Environment setup', () => {
    const tempProvider = anchor.AnchorProvider.env();
    anchor.setProvider(tempProvider);

    const provider = new anchor.AnchorProvider(
        tempProvider.connection,
        new Wallet(payer),
        {skipPreflight: true}
    )
    anchor.setProvider(provider);
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);
    const coreProgram = new CoreProgram(provider.wallet, provider.connection);

    const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
    console.log("destroy", destroyTimestamp);
    let mint: PublicKey;
    let globalCollectionConfigAddress: PublicKey;

    before(async () => {
        console.log("Creating program delegate");
        await burgerProgram.createProgramDelegate();
        console.log("Creating global collection config");
        await coreProgram.createGlobalCollectionConfig();
        globalCollectionConfigAddress = coreProgram.getGlobalCollectionConfigAddress();
        console.log("globalCollectionAddress", globalCollectionConfigAddress.toString());
        const globalCollectionData = await coreProgram.program.account.globalCollectionConfig.fetch(
            globalCollectionConfigAddress);
        mint = PublicKey.findProgramAddressSync(
            [Buffer.from("MINT"),
                globalCollectionData.collectionCounter.toArrayLike(Buffer, "le", 8),
                new BN(0).toArrayLike(Buffer, "le", 8)],
            coreProgram.program.programId)[0];
        console.log("mint", mint.toString());
    });

    it('Mint token', async () => {
        const tx = await burgerProgram.createWhitelistMintTx(
            destroyTimestamp,
            mint,
            globalCollectionConfigAddress
        )

        await sendAndConfirmRawTransaction(
            provider.connection,
            tx,
            provider.publicKey,
            provider.wallet,
            []
        );

        const metadata = await getTokenMetadata(provider.connection, mint);
        console.log("Individual Mint Metadata", metadata);
    });

    it('Transfer token', async () => {
        const tx = await buildNFTTransferTx({
            connection: provider.connection,
            mint: mint,
            source: provider.wallet.publicKey,
            destination: destination,
            payer: payer.publicKey,
        })

        await sendAndConfirmRawTransaction(
                provider.connection,
                tx,
                payer.publicKey,
                undefined,
                [payer]
            );
    });

    it('Renew token', async () => {
        await burgerProgram.renewToken(mint);
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
