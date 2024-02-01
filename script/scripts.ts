import {Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction} from "@solana/web3.js";
import {  loadOrGenerateKeypair } from "../script/utils/helpers";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { BurgerProgram } from "../app/client/burgerProgram";
import * as anchor from "@coral-xyz/anchor";
import { Wallet } from "@coral-xyz/anchor";

import dotenv from "dotenv";
import path from "path";
import {readTextFileFromEndpoint} from "./utils/airdrop";
import {buildNFTTransferTx} from "../app/utils/token2022";
import { EpplexProvider } from "@epplex-xyz/sdk";


// REAADME
// npx ts-node scripts.ts needs to be executed within the script folder
// 0 Modify nMints
// 1 Creates a local keypair in .local_keys
// 2 Uses that to mint
// 2a - might need to uncomment code that does airdrop
// 3 Copy the keypair into Solflare and use for wallet checking

dotenv.config({path: path.resolve(__dirname, ".env.local")})
const fileUrl = process.env.URL;


const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
);

// const mintPool = loadKeypairFromFile("/Users/Mac/Desktop/keypairs/pooPXJECKuyeahBbCat384tAhePkECTPwqs47z9eEQE.json")
const mintPool = loadOrGenerateKeypair("mintPool");
const destroyTimestamp: string = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()



// const provider = new anchor.AnchorProvider(
//     connection,
//     new Wallet(mintPool),
//     {skipPreflight: true}
// )
const epplex = new EpplexProvider(new Wallet(mintPool), connection);

const expiryDate = (Math.floor((new Date()).getTime() / 1000) + 3600).toString()
const metadata = {
    expiryDate: expiryDate,
    name: "(SDK tests) Ephemeral burger",
    symbol: "EP",
    uri: "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo"
}


async function mint(numMints) {
    // Do devnet airdrop
    const airdropSignature = await connection.requestAirdrop(
        mintPool.publicKey, 1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(
        { signature: airdropSignature, ...(await connection.getLatestBlockhash()) }
    );

    for (let i = 0; i < numMints; i++) {
        const mint = Keypair.generate();
        console.log("Item ", i, mint.publicKey.toString());
        const tx = await epplex.createWhitelistMintTx({
            expiryDate: metadata.expiryDate,
            mint: mint,
            name: `${metadata.name} ${i}`,
            symbol: metadata.symbol,
            uri: metadata.uri
        })

        await sendAndConfirmRawTransaction(
            epplex.provider.connection,
            tx,
            epplex.provider.publicKey,
            epplex.provider.wallet,
            [mint]
        );
    }
}

async function mintAndTransfer() {
    // Do devnet airdrop
    const airdropSignature = await connection.requestAirdrop(
        mintPool.publicKey, 1 * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(
        {signature: airdropSignature, ...(await connection.getLatestBlockhash())}
    );

    const content = await readTextFileFromEndpoint(fileUrl);
    if (!content) {
        console.log("Couldn't fetch")
        return null;
    }
    // const content = ["Czdbq7j1UfDRvLUFtyhKexQcoAdQSomh1RJEAcJ66iuD"]

    let i = 0;
    for (const key of content) {
        const mint = Keypair.generate();
        const destination = new PublicKey(key);

        const txCreate = await epplex.createWhitelistMintTx({
            expiryDate: metadata.expiryDate,
            mint: mint,
            name: `${metadata.name} ${i}`,
            symbol: metadata.symbol,
            uri: metadata.uri
        })
        await sendAndConfirmRawTransaction(
            epplex.provider.connection,
            txCreate,
            epplex.provider.publicKey,
            epplex.provider.wallet,
            [mint]
        );
        console.log("Minted item ", i, mint.publicKey.toString());


        const txTransfer = await buildNFTTransferTx({
            connection: epplex.provider.connection,
            mint: mint.publicKey,
            source: mintPool.publicKey,
            destination: destination,
            payer: mintPool.publicKey,
        })
        await sendAndConfirmRawTransaction(
            epplex.provider.connection,
            txTransfer,
            epplex.provider.publicKey,
            epplex.provider.wallet,
            []
        );
        console.log("Transferred item ", i);

        i++;
    }
}


async function main() {
    // await mintAndTransfer()
    // await mint(3)
}

main()






