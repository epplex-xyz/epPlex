import { testPrelude } from './testUtils';
import {Program2} from "../app/client/program2";
import { BN } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { tokenMetadata } from './pda';
describe('epplex Basic API', () => {
    const {
        coreProgram,
        metadataProgram,
        connection,
        wallet
    } = testPrelude();

    const mainProgram = new Program2(wallet, connection);
    const programDelegate = mainProgram.getProgramDelegate();
    const mint = Keypair.generate();
    const payer = wallet.publicKey
    const ata = getAssociatedTokenAddressSync(
        mint.publicKey,
        payer,
        undefined,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const tm = tokenMetadata(mint.publicKey, metadataProgram.programId)

    // In case the program delegate has not been created.
    it("Init delegate", async() => {
        await coreProgram.methods.programDelegateCreate([])
            .accounts({
                programDelegate,
                payer
            })
            .rpc()
    })

    it("Create Global Collection Config", async() => {
        await coreProgram.methods.globalCollectionConfigCreate()
            .accounts({
                globalCollectionConfig: mainProgram.globalCollectionConfig(),
                payer: payer
            })
            .rpc()
    })

    it('Mint epNFT', async () => {
        const tokenCreateTx = await coreProgram.methods
            .tokenMint({
                destroyTimestampOffset: new BN(1000),
                name: "hello",
                symbol: "sm",
                uri: "",
            })
            .accounts({
                mint: mint.publicKey,
                ata,
                tokenMetadata: tm,
                programDelegate: programDelegate,
                payer,
                systemProgram: SystemProgram.programId,
                token22Program: TOKEN_2022_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
                associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
                metadataProgram: metadataProgram.programId
            })
            .transaction()

        const id = await sendAndConfirmRawTransaction(
            connection,
            tokenCreateTx,
            payer,
            wallet,
            [mint]
        );


        console.log("TX Mint epNFT", id)

    });
});
