import * as pda from "./pda";
import { BN } from "@coral-xyz/anchor";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    SYSVAR_RENT_PUBKEY,
    SystemProgram,
} from "@solana/web3.js";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddressSync,
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { testPrelude } from "./testUtils";

describe("ep-mint", () => {
    const { coreProgram, metadataProgram, mintProgram, connection, wallet } =
        testPrelude();

    const collectionName = "Blessed Burgers";
    const collectionSymbol = "BRGR";

    const programDelegate = pda.programDelegate(coreProgram.programId);

    it("shall set up a mint guard", async () => {
        let gccKey = pda.globalCollectionConfig(coreProgram.programId);
        let gcc = await coreProgram.account.globalCollectionConfig.fetch(
            gccKey
        );
        const collectionConfig = pda.collectionConfig(
            gcc.collectionCounter,
            coreProgram.programId
        );
        const mintGuard = pda.mintGuard(
            collectionConfig,
            mintProgram.programId
        );

        console.log(
            "Global Collection Config",
            pda.globalCollectionConfig(coreProgram.programId).toString()
        );
        console.log("Collection config", collectionConfig.toString());

        const collectionMint = Keypair.generate().publicKey;

        try {
            const tx = await mintProgram.methods
                .initMintGuard({
                    collectionRenewalPrice: new BN(100),
                    collectionStandardDuration: 100,
                    collectionGracePeriod: new BN(100),
                    collectionSize: 3,
                    collectionMintPrice: new BN(LAMPORTS_PER_SOL),
                    collectionName: collectionName,
                    collectionSymbol: collectionSymbol,
                })
                .accounts({
                    creator: wallet.publicKey,
                    mintGuard: mintGuard,
                    epplexProgram: coreProgram.programId,
                    collectionMint: collectionMint,
                    collectionConfig: collectionConfig,
                    globalCollectionConfig: gccKey,
                    programDelegate: programDelegate,
                    token22Program: TOKEN_2022_PROGRAM_ID,
                })
                .rpc({ skipPreflight: true });

            console.log("MINT POOL CREATED:", tx);
            // const acc = await core_program.account.collectionConfig.fetch(collectionConfig)
        } catch (e) {
            console.log(e);
        }
        console.log("\n");
    });

    it("shall mint from a collection", async () => {
        const collectionCounter = new BN(0);
        const collectionConfig = pda.collectionConfig(
            collectionCounter,
            coreProgram.programId
        );
        const mintGuard = pda.mintGuard(
            collectionConfig,
            mintProgram.programId
        );

        const tokenMint = Keypair.generate();
        const ata = getAssociatedTokenAddressSync(
            tokenMint.publicKey,
            wallet.publicKey,
            false,
            TOKEN_2022_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );

        try {
            const tx = await mintProgram.methods
                .mintFromCollection()
                .accounts({
                    minter: wallet.publicKey,
                    mintGuard: mintGuard,
                    epplexProgram: coreProgram.programId,
                    collectionConfig: collectionConfig,
                    tokenMint: tokenMint.publicKey,
                    ata: ata,
                    tokenMetadata: pda.tokenMetadata(
                        tokenMint.publicKey,
                        metadataProgram.programId
                    ),
                    programDelegate: programDelegate,
                    rent: SYSVAR_RENT_PUBKEY,
                    token22Program: TOKEN_2022_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                    associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                    metadataProgram: metadataProgram.programId,
                })
                .transaction();

            const id = await sendAndConfirmRawTransaction(
                connection,
                tx,
                wallet.publicKey,
                wallet,
                [tokenMint]
            );

            console.log("MINTED: ", id);
        } catch (e) {
            console.log("err", e);
        }

        console.log("\n");
    });

    it("shall withdraw mint funds from the guard", async () => {
        const collectionCounter = new BN(0);
        const collectionConfig = pda.collectionConfig(
            collectionCounter,
            coreProgram.programId
        );
        const mintGuard = pda.mintGuard(
            collectionConfig,
            mintProgram.programId
        );

        try {
            const tx = await mintProgram.methods
                .withdrawFunds({
                    amount: new BN(LAMPORTS_PER_SOL),
                })
                .accounts({
                    mintGuard: mintGuard,
                    collectionConfig: collectionConfig,
                    systemProgram: SystemProgram.programId,
                })
                .rpc();

            console.log("Withdrew funds: ", tx);
        } catch (e) {
            console.log(e);
        }
    });
});
