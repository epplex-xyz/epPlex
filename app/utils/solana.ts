import { Connection, Keypair, ParsedAccountData, PublicKey, Transaction, TransactionSignature } from "@solana/web3.js";
import {
    AccountLayout,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    createMintToInstruction,
    getAssociatedTokenAddressSync,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { Token22, Token22Layout } from "../client/types/token22";
import { AnchorWallet } from "@solana/wallet-adapter-react";
import { COMMITMENT, CONFIRM_OPTIONS } from "../client/constants";
import { EpNFT, EpNFTLayout, TokenMetadataLayout } from "../client/types/epNFT";

// https://solana.stackexchange.com/questions/107/how-can-i-get-the-owner-wallet-of-an-nft-mint-using-web3-js
export async function getMintOwner(connection: Connection, mint: PublicKey): Promise<PublicKey> {
    const largestAccounts = await connection.getTokenLargestAccounts(mint);
    const largestAccountInfo = await connection.getParsedAccountInfo(
        largestAccounts.value[0].address  //first element is the largest account, assumed with 1
    );

    if (largestAccountInfo.value === null){
        throw Error("Largest account does not exist");
    }

    const owner = (largestAccountInfo.value.data as ParsedAccountData).parsed.info.owner;

    return new PublicKey(owner);

}

async function getToken22AccountInfo(connection: Connection, mint: PublicKey): Promise<Token22> {
    const info = await connection.getAccountInfo(mint);
    return Token22Layout.decode(info!.data);
}

export async function getToken22(
    connection: Connection,
    publicKey: PublicKey
) {
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const token22s: Token22[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        const data = AccountLayout.decode(e.account.data);

        try {
            const mintInfo = await getToken22AccountInfo(connection, data.mint);
            if (mintInfo.destroyTimestampField !== undefined) {
                token22s.push(mintInfo);
            }
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }

    return token22s;
}


async function getEpNFTaccountInfo(connection: Connection, mint: PublicKey): Promise<EpNFT> {
    const info = await connection.getAccountInfo(mint);
    return EpNFTLayout.decode(info!.data);
}
export async function getEpNFTs(
    connection: Connection,
    publicKey: PublicKey
) {
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const epNFTs: EpNFT[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        const data = AccountLayout.decode(e.account.data);

        try {
            const mintInfo = await getEpNFTaccountInfo(connection, data.mint);

            const metadata = mintInfo.metadataAddress;
            const info = await connection.getAccountInfo(metadata);

            if (info === null) {
                throw Error(`Not epNFT ${data.mint.toString()}`);
            }

            epNFTs.push(TokenMetadataLayout.decode(info!.data));
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }

    return epNFTs;
}


export async function sendAndConfirmRawTransaction(
    connection: Connection,
    tx: Transaction,
    feePayer: PublicKey,
    wallet?: AnchorWallet,
    partialSigners: Keypair[] = [],
): Promise<TransactionSignature> {
    const latestBlockhash = await connection.getLatestBlockhash(COMMITMENT);
    tx.recentBlockhash = latestBlockhash.blockhash;
    tx.lastValidBlockHeight = latestBlockhash.lastValidBlockHeight;
    tx.feePayer = feePayer;

    if (partialSigners) {
        partialSigners.forEach((s) => tx.sign(s));
    }

    let txId = "";
    try {
        if (wallet !== undefined) {
            tx = await wallet.signTransaction(tx);
        }

        txId = await connection.sendRawTransaction(tx.serialize(), CONFIRM_OPTIONS);
        console.log("Tx id", txId);

        const res = (
            await connection.confirmTransaction(
                {
                    signature: txId,
                    blockhash: latestBlockhash.blockhash,
                    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
                },
                COMMITMENT
            )
        );

        if (res.value.err) {
            // For some reason this is not logged
            console.log(`Raw transaction ${txId} failed (${JSON.stringify(res.value.err)})`);
            throw res.value.err;
        }
    } catch (e: any) {
        console.log("Caught TX error", e);
    }

    return txId;
}

export async function mint(connection: Connection, mint: PublicKey, payer: Keypair) {
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey, // owner
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    const signature = await mintTo(
        connection,
        payer,
        mint,
        fromTokenAccount.address,
        payer,
        1,
        [],
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("tx", signature);
}

export function mintToIx(mint: PublicKey, payer: PublicKey) {
    const associatedToken = getAssociatedTokenAddressSync(
        mint,
        payer,
        false,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const createAccountIx = createAssociatedTokenAccountInstruction(
        payer,
        associatedToken,
        payer,
        mint,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const mintToIx = createMintToInstruction(
        mint,
        associatedToken,
        payer,
        1,
        [],
        TOKEN_2022_PROGRAM_ID
    );

    return [createAccountIx, mintToIx];
}

export function explorerURL({
    address,
    txSignature,
    cluster,
}: {
    address?: string;
    txSignature?: string;
    cluster?: "devnet" | "testnet" | "mainnet" | "mainnet-beta";
}) {
    let baseUrl: string;
    //
    if (address) baseUrl = `https://explorer.solana.com/address/${address}`;
    else if (txSignature) baseUrl = `https://explorer.solana.com/tx/${txSignature}`;
    else return "[unknown]";

    // auto append the desired search params
    const url = new URL(baseUrl);
    url.searchParams.append("cluster", cluster || "devnet");
    return url.toString() + "\n";
}
