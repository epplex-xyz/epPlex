import { Connection, PublicKey } from "@solana/web3.js";
import { AccountLayout, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Token22Layout, Token22 } from "../../client/token22";


async function getAccountInfo(connection: Connection, mint: PublicKey): Promise<Token22> {
    const info = await connection.getAccountInfo(mint);
    return Token22Layout.decode(info!.data);
}

export async function getTokenBalances(
    connection: Connection,
    publicKey: PublicKey
) {
    const allTokenAccounts = await connection.getTokenAccountsByOwner(publicKey, { programId: TOKEN_2022_PROGRAM_ID });

    const epNFTs: Token22[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        const data = AccountLayout.decode(e.account.data);
        try {
            const mintInfo = await getAccountInfo(connection, data.mint);
            if (mintInfo.destroyTimestampField !== undefined) {
                epNFTs.push(mintInfo);
            }
        } catch (e) {
            console.log("Failed to decode", e);
        }
    }

    return epNFTs;
}