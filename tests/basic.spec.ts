import { testPrelude } from './testUtils';
import {Program2} from "../app/client/program2";
import { BN } from "@coral-xyz/anchor";
import { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import * as pda from './pda';

describe('epplex Basic API', () => {
    const {
        coreProgram,
        burgerProgram,
        connection,
        wallet
    } = testPrelude();

    const burgerProgramDelegate = pda.burgerProgramDelegate(burgerProgram.programId);
    const mint = Keypair.generate();
    const payer = wallet.publicKey
    const ata = getAssociatedTokenAddressSync(
        mint.publicKey,
        payer,
        undefined,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const destroyTimestamp: string = "1706020091"

    // const tm = tokenMetadata(mint.publicKey, metadataProgram.programId)


    // it("Create burger delegate", async() => {
    //     try {
    //         const tx = await burgerProgram.methods.programDelegateCreate({})
    //             .accounts({
    //                 programDelegate: burgerProgramDelegate,
    //                 payer: payer,
    //                 systemProgram: SystemProgram.programId
    //             })
    //             .rpc({skipPreflight: true})
    //
    //         console.log("tx", tx)
    //     } catch (e) {
    //         console.log("error", e)
    //     }
    //
    //     console.log("\n")
    // })

    // it('Mint whitelist epNFT', async () => {
    //     // await burgerProgram.createWhitelistMint(destroyTimestamp)
    //
    //     const tokenCreateTx = await burgerProgram.methods
    //         .whitelistMint({
    //             name: "hello",
    //             symbol: "sm",
    //             uri: "",
    //             destroyTimestamp: "1705952387"
    //         })
    //         .accounts({
    //             mint: mint.publicKey,
    //             ata,
    //             // tokenMetadata: tm,
    //             permanentDelegate: burgerProgramDelegate,
    //             payer,
    //             systemProgram: SystemProgram.programId,
    //             token22Program: TOKEN_2022_PROGRAM_ID,
    //             rent: SYSVAR_RENT_PUBKEY,
    //             associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
    //             epplexCore: coreProgram.programId
    //             // metadataProgram: metadataProgram.programId
    //         })
    //         .transaction()
    //
    //     const id = await sendAndConfirmRawTransaction(
    //         connection,
    //         tokenCreateTx,
    //         payer,
    //         wallet,
    //         [mint]
    //     );
    //
    //     console.log("TX Mint epNFT", id)
    //     console.log("\n")
    // });
});
