import { PublicKey } from "@solana/web3.js"
import * as anchor from "@coral-xyz/anchor";

const programId = new PublicKey("DWQ12BSvpNq6AxX18Xgm72avoCT8nL8G7R886NeiLFeN")
const epplexId = new PublicKey("7N839QzgShmkazepHcHx87u4gg29jTsYYeY8rNV7XffR")

export function programDelegate() {
    const [programDelegatePDA, programDelegateBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("PROGRAM_DELEGATE"),
        ],
        epplexId
      )
      return programDelegatePDA;
}

export function mintGuard(collectionName: string) {
    const [mintGuardPDA, mintGuardBump] = PublicKey.findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("guard"),
          anchor.utils.bytes.utf8.encode(collectionName),
  
        ],
        programId
      )
      return mintGuardPDA
}


export function collectionConfig(collectionName: string) {
    const [collectionConfigPDA, collectionConfigBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("CONFIG"), Buffer.from(collectionName)],
        epplexId
      )

      return collectionConfigPDA
}