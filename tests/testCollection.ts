import { BurgerProgram } from "../app/client/burgerProgram";
import {BN, Wallet} from "@coral-xyz/anchor";
import {Keypair, PublicKey} from "@solana/web3.js";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { buildNFTTransferTx } from "../app/utils/token2022";
import { loadKeypairFromFile } from "../script/utils/helpers";
import {CoreProgram} from "../app/client/coreProgram";
import {getTokenMetadata} from "@solana/spl-token";
import {mintTokenIntoCollection} from "./mintUtils";
import * as anchor from "@coral-xyz/anchor";


const secretKeypair = loadKeypairFromFile("/home/fzzyyti/.config/solana/test.json")
const mintPool = loadKeypairFromFile("/home/fzzyyti/.config/solana/mint.json")

describe('Test Collection', () => {
    const tempProvider = anchor.AnchorProvider.env();
    anchor.setProvider(tempProvider);

    const provider = new anchor.AnchorProvider(
        tempProvider.connection,
        new Wallet(secretKeypair),
        {skipPreflight: true}
    )
    anchor.setProvider(provider);
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);
    const coreProgram = new CoreProgram(provider.wallet, provider.connection);


    const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
    console.log("destroy", destroyTimestamp);

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
        const [collectionConfigAddress, _bump] = PublicKey.findProgramAddressSync(
            [Buffer.from("CONFIG"),
                globalCollectionData.collectionCounter.toArrayLike(Buffer, "le", 8)],
            coreProgram.program.programId
        )
        console.log("collectionConfigAddress", collectionConfigAddress.toString());

        await coreProgram.createCollection(collectionConfigAddress, burgerProgram.getProgramDelegate());

        // Mint 10 tokens into the collection
        for (let i = 0; i < 10; i++) {
            const mint = await mintTokenIntoCollection(
                provider,
                burgerProgram,
                coreProgram,
                globalCollectionData.collectionCounter,
                destroyTimestamp);
            const metadata = await getTokenMetadata(provider.connection, mint);
            console.log("metadata", metadata);
        }

    });

    // it('Transfer token', async () => {
    //     const tx = await buildNFTTransferTx({
    //         connection: provider.connection,
    //         mint: mint.publicKey,
    //         source: provider.wallet.publicKey,
    //         destination: mintPool.publicKey,
    //         payer: secretKeypair.publicKey,
    //     })
    //
    //     await sendAndConfirmRawTransaction(
    //         provider.connection,
    //         tx,
    //         secretKeypair.publicKey,
    //         undefined,
    //         [secretKeypair]
    //     );
    // });
    //
    // it('Renew token', async () => {
    //     await burgerProgram.renewToken(mint.publicKey)
    // });

});
