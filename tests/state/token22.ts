import {PublicKey} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

type FixedLengthArray<T, L extends number> = L extends L
  ? number[] extends ((
      ...args: [...Array<L>]
    ) => void)
    ? T[]
    : [...Array<L>]
  : never;

interface Token22 {
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
    uri: string
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
]);


// export const MintLayout = struct<RawMint>([
//     u32('mintAuthorityOption'),
//     publicKey('mintAuthority'),
//     u64('supply'),
//     u8('decimals'),
//     bool('isInitialized'),
//     u32('freezeAuthorityOption'),
//     publicKey('freezeAuthority'),
// ]);
// mintCloseAuth is 32 byes + 4 console.log("closeauth", MINT_CLOSE_AUTHORITY_SIZE);
// perm delegate is 32 byts + 4 console.log("perma", PERMANENT_DELEGATE_SIZE);
// tokenmetadata is 64 + 4