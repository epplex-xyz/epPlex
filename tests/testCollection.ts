import * as anchor from "@coral-xyz/anchor";
import { BurgerProgram } from "../app/client/burgerProgram";
import { Wallet } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { buildNFTTransferTx } from "../app/utils/token2022";
import { loadKeypairFromFile } from "../script/utils/helpers";


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


    const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
    console.log("destroy", destroyTimestamp);
    const mint = Keypair.generate();

    it("Create burger delegate ", async() => {
        await burgerProgram.createProgramDelegate();
    })

    it('Mint token', async () => {
        const tx = await burgerProgram.createWhitelistMintTx(
            destroyTimestamp,
            mint
        )
        console.log("rpc", provider.connection.rpcEndpoint);
        await sendAndConfirmRawTransaction(
            provider.connection,
            tx,
            provider.publicKey,
            provider.wallet,
            [mint]
        );
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

});
