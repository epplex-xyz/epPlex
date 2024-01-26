import { Connection, Keypair, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import {  loadOrGenerateKeypair } from "../script/utils/helpers";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { BurgerProgram } from "../app/client/burgerProgram";
import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

// REAADME
// 0 Modify nMints
// 1 Creates a local keypair in .local_keys
// 2 Uses that to mint
// 2a - might need to uncomment code that does airdrop
// 3 Copy the keypair into Solflare and use for wallet checking

const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
);

const mintPool = loadOrGenerateKeypair("mintPool");
const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
const nMints = 3;

async function main() {
    // const mintPool = loadKeypairFromFile("/Users/Mac/Desktop/keypairs/pooPXJECKuyeahBbCat384tAhePkECTPwqs47z9eEQE.json")

    // Do devnet airdrop
    // const airdropSignature = await connection.requestAirdrop(
    //     mintPool.publicKey, 1 * LAMPORTS_PER_SOL
    // );
    // await connection.confirmTransaction(
    //     { signature: airdropSignature, ...(await connection.getLatestBlockhash()) }
    // );

    const provider = new anchor.AnchorProvider(
        connection,
        new Wallet(mintPool),
        {skipPreflight: true}
    )
    const burgerProgram = new BurgerProgram(provider.wallet, provider.connection);
    // Could batch txes but 1 tx is too large
    // let txs: Transaction[] = []
    // let signers: Keypair[] = []
    for (let i = 0; i < nMints; i++) {
        const mint = Keypair.generate();
        console.log("Item ", i, mint.publicKey.toString());
        const tx = await burgerProgram.createWhitelistMintTx(
            destroyTimestamp,
            mint
        )
        // txs.push(tx)
        // signers.push(mint)

        await sendAndConfirmRawTransaction(
            provider.connection,
            tx,
            provider.publicKey,
            provider.wallet,
            [mint]
        );
    }


}

main();