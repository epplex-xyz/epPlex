import * as anchor from "@coral-xyz/anchor";
import * as pda from "./pda";
import { Program, BN } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { sendAndConfirmRawTransaction } from "../app/utils/solana";
import { EpplexMint } from "../target/types/epplex_mint";
import { EpplexMetadata } from "../target/types/epplex_metadata";
import { EpplexCore } from "../target/types/epplex_core";

describe("ep-mint",() => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)
  const mint_program = anchor.workspace.EpplexMint as Program<EpplexMint>;
  const core_program = anchor.workspace.EpplexCore as Program<EpplexCore>
  const metadata_program = anchor.workspace.EpplexMetadata as Program<EpplexMetadata>

  const collectionName = "Blessed Burgers"
  const collectionSymbol = "BRGR"

  const programDelegate = pda.programDelegate(core_program.programId)

  it("shall set up a mint guard", async() => {

    let gccKey = pda.globalCollectionConfig(core_program.programId)
    let gcc = await core_program.account.globalCollectionConfig.fetch(gccKey)
    const collectionConfig = pda.collectionConfig(gcc.collectionCounter, core_program.programId)
    const mintGuard = pda.mintGuard(collectionConfig, mint_program.programId)


    console.log(pda.globalCollectionConfig(core_program.programId))
    console.log(gcc)

    const collectionMint = Keypair.generate().publicKey
  
    try {
      const tx = await mint_program.methods.initMintGuard({
        collectionRenewalPrice: new BN(100),
        collectionStandardDuration: 100,
        collectionGracePeriod: new BN(100),
        collectionSize: 3,
        collectionMintPrice: new BN(LAMPORTS_PER_SOL),
        collectionName: collectionName,
        collectionSymbol: collectionSymbol
      })
      .accounts({
        creator: provider.wallet.publicKey,
        mintGuard: mintGuard,
        epplexProgram: core_program.programId,
        collectionMint: collectionMint,
        collectionConfig: collectionConfig,
        globalCollectionConfig: gccKey,
        programDelegate: programDelegate,
        token22Program: TOKEN_2022_PROGRAM_ID,
      })
      .rpc({skipPreflight: true});
      console.log("MINT POOL CREATED:", tx)
      const acc = await core_program.account.collectionConfig.fetch(collectionConfig)
      console.log("COLLECTION CONFIG: ", acc)

    } catch(e) {
      console.log(e)
    }
  
  })

  it("shall mint from a collection", async() => {
    
    const collectionCounter = new BN(0);
    const collectionConfig = pda.collectionConfig(collectionCounter, core_program.programId)
    const mintGuard = pda.mintGuard(collectionConfig, mint_program.programId)

    const tokenMint = Keypair.generate()
    const ata = getAssociatedTokenAddressSync(
        tokenMint.publicKey,
        provider.wallet.publicKey,
        false,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
    );


    try {
      const tx = await mint_program.methods.mintFromCollection()
          .accounts({
            minter: provider.wallet.publicKey,
            mintGuard: mintGuard,
            epplexProgram: core_program.programId,
            collectionConfig: collectionConfig,
            tokenMint: tokenMint.publicKey,
            ata: ata,
            tokenMetadata: pda.tokenMetadata(tokenMint.publicKey, metadata_program.programId),
            programDelegate: programDelegate,
            rent: SYSVAR_RENT_PUBKEY,
            token22Program: TOKEN_2022_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedToken: ASSOCIATED_TOKEN_PROGRAM_ID,
            metadataProgram: metadata_program.programId
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

  it("shall withdraw mint funds from the guard", async() => {
    const collectionCounter = new BN(0);
    const collectionConfig = pda.collectionConfig(collectionCounter, core_program.programId)
    const mintGuard = pda.mintGuard(collectionConfig, mint_program.programId)

    try {
      const tx = await mint_program.methods.withdrawFunds({
        amount: new BN(LAMPORTS_PER_SOL)
      })
      .accounts({
        mintGuard: mintGuard,
        collectionConfig: collectionConfig,
        systemProgram: SystemProgram.programId
      })
      .rpc()

      console.log("Withdrew funds: ", tx)
    } catch(e) {
      console.log(e)
    }

  })

});
