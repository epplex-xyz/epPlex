import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EpMint } from "../target/types/ep_mint";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Ephemerality } from "../target/types/ephemerality";

describe("ep-mint", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local()
  anchor.setProvider(provider)
  const mint_program = anchor.workspace.EpMint as Program<EpMint>;
  const epplex_program = anchor.workspace.Ephemerality as Program<Ephemerality>

  const programId = new PublicKey("DWQ12BSvpNq6AxX18Xgm72avoCT8nL8G7R886NeiLFeN")
  const epplex_id = new PublicKey("7N839QzgShmkazepHcHx87u4gg29jTsYYeY8rNV7XffR")
  const token2022_id = new PublicKey("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")


  it("shall init epplex", async() => {

    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("PROGRAM_DELEGATE"),
      ],
      epplex_id
    )

    await epplex_program.methods.programDelegateCreate([])
    .accounts({
      programDelegate: programDelegatePDA,
      payer: provider.wallet.publicKey
    })
    .rpc()

  })

  it("shall set up a mint pool", async() => {

    const collection_name = "burgers"

    const [mintPoolPDA, mintPoolBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("pool"),
        anchor.utils.bytes.utf8.encode(collection_name),

      ],
      programId
    )

    const collectionMint = Keypair.generate().publicKey

    const [collectionConfigPDA, collectionConfigBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("CONFIG"), Buffer.from(collection_name)],
      epplex_id
    )

    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("PROGRAM_DELEGATE"),
      ],
      epplex_id
    )

    const tx = await mint_program.methods.initMintPool(collection_name)
    .accounts({
      creator: provider.wallet.publicKey,
      mintPool: mintPoolPDA,
      epplexProgram: epplex_id,
      collectionMint: collectionMint,
      collectionConfig: collectionConfigPDA,
      programDelegate: programDelegatePDA,
      token22Program: token2022_id,
    })
    .rpc();

    console.log("MINT POOL CREATED:", tx)

    const acc = await epplex_program.account.collectionConfig.fetch(collectionConfigPDA)

    console.log("COLLECTION CONFIG: ", acc)

  })

});
