import {PublicKey} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import { RawMint } from "@solana/spl-token/src/state/mint";

type FixedLengthArray<T, L extends number> = L extends L
    ? number[] extends ((
            ...args: [...Array<L>]
        ) => void)
        ? T[]
        : [...Array<L>]
    : never;

export interface EpNFT extends RawMint, EpNFTExtensions {}

export interface EpNFTExtensions extends CloseAuthExtension, PermanentDelegateExtension, MetadataPointerExtension {}

export interface CloseAuthExtension {
    dunno1: FixedLengthArray<any, 5>,
    closeAuthority: PublicKey,
}

export interface PermanentDelegateExtension {
    permanentDelegateOption: 1 | 0,
    permanentDelegate: PublicKey,
    dunno3: FixedLengthArray<any, 4>,
    dunno4: PublicKey,
    dunno5: PublicKey,
}

export interface MetadataPointerExtension {
    metadataPointerAuthorityOption: 1 | 0,
    metadataPointerAuthority: PublicKey,
    metadataAddress: PublicKey
}

// /** Buffer layout for de/serializing a mint */
export const EpNFTLayout = borsh.struct<EpNFTExtensions>([
    borsh.array(borsh.u8(), 5, "dunno1"),
    borsh.publicKey("closeAuthority"),

    borsh.u32("permanentDelegateOption"),
    borsh.publicKey("permanentDelegate"),

    borsh.u32("metadataPointerAuthorityOption"),
    borsh.publicKey("metadataPointerAuthority"),
    borsh.publicKey("metadataAddress"),
]);



export interface TokenMetadata {
    metadataAuthorityOption: 1 | 0,
    metadataAuthority: PublicKey,
    mint: PublicKey,
    name: string,
    symbol: string,
    uri: string,
    // How to corrobate these two
    dunno7: FixedLengthArray<any, 4>
    expirationDate: string,
    expirationDateValue: string
}

// /** Buffer layout for de/serializing a mint */
export const TokenMetadataLayout = borsh.struct<TokenMetadata>([
    // TokenMetadata
    borsh.u8("metadataAuthorityOption"),
    borsh.publicKey("metadataAuthority"),
    borsh.publicKey("mint"),
    borsh.str("name"),
    borsh.str("symbol"),
    borsh.str("uri"),
    borsh.array(borsh.u8(), 4, "dunno7"),
    borsh.str("expirationDate"),
    borsh.str("expirationDateValue"),
]);