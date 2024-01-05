import * as anchor from "@coral-xyz/anchor";
import * as pda from "./pda";
import { Program, BN } from "@coral-xyz/anchor";
import { EpMint } from "../target/types/ep_mint";
import { Connection, Keypair, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram } from "@solana/web3.js";
import { Ephemerality } from "../target/types/ephemerality";

describe("ep-mint", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)
  const mint_program = anchor.workspace.EpMint as Program<EpMint>;
  const epplex_program = anchor.workspace.Ephemerality as Program<Ephemerality>

  const program_id = new PublicKey("DWQ12BSvpNq6AxX18Xgm72avoCT8nL8G7R886NeiLFeN")
  const epplex_id = new PublicKey("7N839QzgShmkazepHcHx87u4gg29jTsYYeY8rNV7XffR")
  const token2022_id = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
  const collectionName = "burgers"

  const con = new Connection("http://127.0.0.1:8899");

  it("shall init epplex", async() => {

    await epplex_program.methods.programDelegateCreate([])
    .accounts({
      programDelegate: pda.programDelegate(),
      payer: provider.wallet.publicKey
    })
    .rpc()

  })

  it("shall set up a mint guard", async() => {
    const collectionMint = Keypair.generate().publicKey

    const tx = await mint_program.methods.initMintGuard({
      collectionRenewalPrice: new BN(100),
      collectionStandardDuration: 100,
      collectionGracePeriod: new BN(100),
      collectionSize: 10000,
      collectionName: collectionName
    })
    .accounts({
      creator: provider.wallet.publicKey,
      mintGuard: pda.mintGuard(collectionName),
      epplexProgram: epplex_id,
      collectionMint: collectionMint,
      collectionConfig: pda.collectionConfig(collectionName),
      programDelegate: pda.programDelegate(),
      token22Program: token2022_id,
    })
    .rpc();

    console.log("MINT POOL CREATED:", tx)

    const acc = await epplex_program.account.collectionConfig.fetch(pda.collectionConfig(collectionName))

    console.log("COLLECTION CONFIG: ", acc)

  })

  it("shall mint from a collection", async() => {
    
    const tokenMint = Keypair.generate().publicKey

    const tx = await mint_program.methods.mintFromCollection(collectionName)
    .accounts({
      minter: provider.wallet.publicKey,
      mintGuard: pda.mintGuard(collectionName),
      epplexProgram: epplex_id,
      collectionConfig: pda.collectionConfig(collectionName),
      tokenMint: tokenMint,
      programDelegate: pda.programDelegate(),
      rent: SYSVAR_RENT_PUBKEY,
      token22Program: token2022_id,
      systemProgram: SystemProgram.programId,
    })
    .rpc()

    console.log("MINTED: ", tx)


  })

});
