import { Connection, PublicKey } from "@solana/web3.js";
import { AccountLayout, MINT_SIZE, MintLayout, TOKEN_2022_PROGRAM_ID, getTokenMetadata } from "@solana/spl-token";
import { Token2022Interface, Token22Layout } from "../client/types/token2022Interface";
import { EpNFT, EpNFTLayout, TokenMetadataLayout } from "../client/types/epNFT";
import { Program2 } from "../client/program2";
import {TokenMetadata} from  "@solana/spl-token-metadata";

// Old decoding method
export async function getToken22(
    connection: Connection,
    publicKey: PublicKey
): Promise<Token2022Interface[]> {
    // Get all Token2022s of owner
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const token22s: Token2022Interface[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        // Get raw data
        const data = AccountLayout.decode(e.account.data);

        try {
            const mintInfo = await connection.getAccountInfo(data.mint);
            const decoded = Token22Layout.decode(mintInfo!.data);
            if (decoded.destroyTimestampField !== undefined) {
                token22s.push(decoded);
            }
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }

    return token22s;
}

export async function myGetTokenMetadata(
    connection: Connection,
    publicKey: PublicKey
): Promise<TokenMetadata[]> {
    // Get all Token2022s of owner
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const tokenMetadata: TokenMetadata[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        // Get raw data
        const data = AccountLayout.decode(e.account.data);

        try {
            const metadata = await getTokenMetadata(connection, data.mint);

            console.log("metadata", metadata);
            if (metadata !== null) {
                tokenMetadata.push(metadata);
            }
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }

    return tokenMetadata;
}


// Using metadataPointer address
async function getEpNFTaccountInfo(connection: Connection, mint: PublicKey): Promise<EpNFT> {
    const info = await connection.getAccountInfo(mint);
    const data = info!.data;

    const tokenBaseData = data.slice(0, MINT_SIZE);
    const mintDetailsData = data.slice(MINT_SIZE + 83);

    const tokenBase = MintLayout.decode(tokenBaseData);
    const mintDetails = EpNFTLayout.decode(mintDetailsData);

    return { ...tokenBase, ...mintDetails};
}

// Using metadataPointer address
export async function getEpNFTs(
    connection: Connection,
    publicKey: PublicKey
) {
    // Get all Token2022s of owner
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const epNFTs: TokenMetadata[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        // Get raw data
        const data = AccountLayout.decode(e.account.data);

        try {
            // Get metadata pointer address
            const mintInfo = await getEpNFTaccountInfo(connection, data.mint);
            const metadata = mintInfo.metadataAddress;

            // Fetch the pda
            const pda = Program2.staticGetTokenMetadata(data.mint).toString();

            // Check if they equal - means it is from our program
            const isEpNFT = metadata.toString() === pda;
            if (!isEpNFT) {
                throw Error(`1 Not epNFT ${data.mint.toString()}`);
            }

            // Decode the data on the metadata account
            const info = await connection.getAccountInfo(metadata);
            if (info === null) {
                throw Error(`2 Not epNFT ${data.mint.toString()}`);
            }

            const pdaData = info!.data;
            const discriminant = pdaData.slice(8);
            const decoded = TokenMetadataLayout.decode(discriminant);
            console.log("decoded", decoded);

            epNFTs.push(decoded);
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }
    console.log();

    return epNFTs;
}
