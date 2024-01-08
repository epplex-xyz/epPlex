import * as anchor from "@coral-xyz/anchor";
import * as pda from "./pda";
import { Program, BN } from "@coral-xyz/anchor";
import { EpMint } from "../target/types/ep_mint";
import { Connection, Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { Ephemerality } from "../target/types/ephemerality";
import { tokenMetadata } from "./pda";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";

describe("ep-mint", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)
  const mint_program = anchor.workspace.EpMint as Program<EpMint>;
  const epplex_program = anchor.workspace.Ephemerality as Program<Ephemerality>

  const collectionName = "BRGR"

  const programDelegate = pda.programDelegate(epplex_program.programId)
  const collectionConfig = pda.collectionConfig(collectionName, epplex_program.programId)
  const mintGuard = pda.mintGuard(collectionName, mint_program.programId)

  // it("shall init epplex", async() => {
  //
  //   const tx = await epplex_program.methods.programDelegateCreate([])
  //   .accounts({
  //     programDelegate,
  //     payer: provider.wallet.publicKey
  //   })
  //   .rpc(options)
  //
  //   console.log("program delegate ID", tx)
  // })
  //
  // it("shall set up a mint guard", async() => {
  //   const collectionMint = Keypair.generate().publicKey
  //
  //   const tx = await mint_program.methods.initMintGuard({
  //     collectionRenewalPrice: new BN(100),
  //     collectionStandardDuration: 100,
  //     collectionGracePeriod: new BN(100),
  //     collectionSize: 10000,
  //     collectionName: collectionName
  //   })
  //   .accounts({
  //     creator: provider.wallet.publicKey,
  //     mintGuard: mintGuard,
  //     epplexProgram: epplex_program.programId,
  //     collectionMint: collectionMint,
  //     collectionConfig: collectionConfig,
  //     programDelegate,
  //     token22Program: TOKEN_2022_PROGRAM_ID,
  //   })
  //   .rpc(options);
  //
  //   console.log("MINT POOL CREATED:", tx)
  //
  //   const acc = await epplex_program.account.collectionConfig.fetch(collectionConfig)
  //   console.log("COLLECTION CONFIG: ", acc)
  //
  // })

  it("shall mint from a collection", async() => {
    
    const tokenMint = Keypair.generate()
    const ata = getAssociatedTokenAddressSync(
        tokenMint.publicKey,
        provider.wallet.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );


    try {
      const tx = await mint_program.methods.mintFromCollection(collectionName)
          .accounts({
            minter: provider.wallet.publicKey,
            mintGuard: mintGuard,
            epplexProgram: epplex_program.programId,
            collectionConfig: collectionConfig,
            tokenMint: tokenMint.publicKey,
            ata: ata,
            tokenMetadata: pda.tokenMetadata(tokenMint.publicKey, epplex_program.programId),
            programDelegate: programDelegate,
            rent: SYSVAR_RENT_PUBKEY,
            token22Program: TOKEN_2022_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .transaction()

        const id = await sendAndConfirmRawTransaction(
            provider.connection, tx, provider.wallet.publicKey, provider.wallet, [tokenMint]
        );

      console.log("MINTED: ", id)

    } catch (e) {
      console.log("err", e)
    }


  })

});
