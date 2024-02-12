import { PublicKey } from "@solana/web3.js"
import * as anchor from "@coral-xyz/anchor";


export function burgerProgramDelegate(programId: PublicKey) {
    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
        [
            anchor.utils.bytes.utf8.encode("BURGER_DELEGATE"),
        ],
        programId
    )
    return programDelegatePDA;
}

export function programDelegate(programId: PublicKey) {
    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("PROGRAM_DELEGATE"),
        ],
        programId
      )
      return programDelegatePDA;
}

export function mintGuard(collectionConfig: PublicKey, programId: PublicKey) {
    const [mintGuardPDA, mintGuardBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("GUARD"),
          collectionConfig.toBuffer()
        ],
        programId
      )
      return mintGuardPDA
}


export function collectionConfig(counter: anchor.BN, programId: PublicKey) {
    const [collectionConfigPDA, collectionConfigBump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("CONFIG"),
          Uint8Array.of(...counter.toArray('le', 8))
        ],
        programId
      )

      return collectionConfigPDA
}

export function globalCollectionConfig(programId: PublicKey): PublicKey {
  const [globalCollectionConfig] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("GLOBAL_COLLECTION")
      ],
      programId
  );
  return globalCollectionConfig;
}

export function tokenMetadata(mint: PublicKey, programId: PublicKey) {
    const [metadata, metadataBump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("metadata"),
          mint.toBuffer()
        ],
        programId
    )

    return metadata
}