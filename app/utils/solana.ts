import {
    Commitment,
    Connection,
    Keypair, ParsedAccountData,
    PublicKey,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {
    AccountLayout,
    createMintToInstruction,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    createAssociatedTokenAccountInstruction,
    ASSOCIATED_TOKEN_PROGRAM_ID, getAccount, TokenAccountNotFoundError, TokenInvalidAccountOwnerError,
} from "@solana/spl-token";
import { Token22Layout, Token22 } from "../client/types/token22";
import { AnchorWallet } from "@solana/wallet-adapter-react";
import { COMMITMENT, CONFIRM_OPTIONS } from "../client/constants";

export async function tryCreateATAIx2(
    connection: Connection,
    payer: PublicKey,
    owner: PublicKey,
    mint: PublicKey,
    allowOwnerOffCurve = false,
    commitment: Commitment = COMMITMENT,
    programId = TOKEN_2022_PROGRAM_ID,
    associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID
): Promise<[TransactionInstruction, PublicKey] | PublicKey | undefined> {
    const ata = getAssociatedTokenAddressSync(mint, owner, allowOwnerOffCurve, programId, associatedTokenProgramId);

    try {
        await getAccount(connection, ata, commitment, programId);
        console.log(`Token account already exists: ${ata.toString()} for token ${mint.toString()}`);
        return ata;
    } catch (error: unknown) {
        // TokenAccountNotFoundError can be possible if the associated address has already received some lamports,
        // becoming a system account. Assuming program derived addressing is safe, this is the only case for the
        // TokenInvalidAccountOwnerError in this code path.
        if (error instanceof TokenAccountNotFoundError || error instanceof TokenInvalidAccountOwnerError) {
            const ix = createAssociatedTokenAccountInstruction(
                payer,
                ata,
                owner,
                mint,
                programId,
                associatedTokenProgramId
            );
            return [ix, ata];
        } else {
            throw error;
        }
    }
}

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

    const epNFTs: Token22[] = [];
    for (const [_, e] of allTokenAccounts.value.entries()) {
        const data = AccountLayout.decode(e.account.data);
        try {
            const mintInfo = await getToken22AccountInfo(connection, data.mint);
            if (mintInfo.destroyTimestampField !== undefined) {
                epNFTs.push(mintInfo);
            }
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
        partialSigners.forEach((s) => tx.partialSign(s));
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