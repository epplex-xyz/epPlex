import {PublicKey} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

type FixedLengthArray<T, L extends number> = L extends L
  ? number[] extends ((
      ...args: [...Array<L>]
    ) => void)
    ? T[]
    : [...Array<L>]
  : never;

export interface Token22 {
    mintAuthorityOption: 1 | 0;
    mintAuthority: PublicKey;
    supply: bigint;
    decimals: number;
    isInitialized: boolean;
    freezeAuthorityOption: 1 | 0;
    freezeAuthority: PublicKey;
    padding: FixedLengthArray<any, 83>,
    dunno1: FixedLengthArray<any, 5>,
    closeAuthority: PublicKey,
    dunno2: FixedLengthArray<any, 4>,
    permanentDelegate: PublicKey,
    dunno3: FixedLengthArray<any, 4>,
    dunno4: PublicKey,
    dunno5: PublicKey,
    dunno6: FixedLengthArray<any, 4>,
    metadataPointerAuthority: PublicKey,
    metadataAddress: PublicKey
    // TokenMetadata
    name: string,
    symbol: string,
    uri: string,
    // How to corrobate these two
    dunno7: FixedLengthArray<any, 4>
    destroyTimestampField: string,
    destroyTimestampValue: string
}

// /** Buffer layout for de/serializing a mint */
export const Token22Layout = borsh.struct<Token22>([
    borsh.u32('mintAuthorityOption'),
    borsh.publicKey('mintAuthority'),
    borsh.u64('supply'),
    borsh.u8('decimals'),
    borsh.bool('isInitialized'),
    borsh.u32('freezeAuthorityOption'),
    borsh.publicKey('freezeAuthority'),
    borsh.array(borsh.u8(), 83, "padding"),
    borsh.array(borsh.u8(), 5, "dunno1"),
    borsh.publicKey("closeAuthority"),
    borsh.array(borsh.u8(), 4, "dunno2"),
    borsh.publicKey("permanentDelegate"),
    borsh.array(borsh.u8(), 4, "dunno3"),
    borsh.publicKey("dunno4"),
    borsh.publicKey("dunno5"), // mint address
    borsh.array(borsh.u8(), 4, "dunno6"),
    borsh.publicKey("metadataPointerAuthority"),
    borsh.publicKey("metadataAddress"),
    // TokenMetadata
    borsh.str("name"),
    borsh.str("symbol"),
    borsh.str("uri"),
    borsh.array(borsh.u8(), 4, "dunno7"),
    borsh.str("destroyTimestampField"),
    borsh.str("destroyTimestampValue"),
]);