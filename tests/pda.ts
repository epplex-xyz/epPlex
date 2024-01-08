import { PublicKey } from "@solana/web3.js"
import * as anchor from "@coral-xyz/anchor";


export function programDelegate(programId: PublicKey) {
    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("PROGRAM_DELEGATE"),
        ],
        programId
      )
      return programDelegatePDA;
}

export function mintGuard(collectionName: string, programId: PublicKey) {
    const [mintGuardPDA, mintGuardBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("guard"),
          anchor.utils.bytes.utf8.encode(collectionName),
  
        ],
        programId
      )
      return mintGuardPDA
}


export function collectionConfig(collectionName: string, programId: PublicKey) {
    const [collectionConfigPDA, collectionConfigBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("CONFIG"), Buffer.from(collectionName)],
        programId
      )

      return collectionConfigPDA
}

export function tokenMetadata(mint: PublicKey, programId: PublicKey) {
    const [metadata, metadataBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"), programId.toBuffer(), mint.toBuffer()],
        programId
    )

    return metadata
}