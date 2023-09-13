
import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

/**
 * TODO: add
 *
 * @param metadata
 * @param updateAuthority
 * @param mint
 * @param mintAuthority
 *
 * @return Instruction to add to a transaction
 */
export function metadataInstruction(
    metadata: PublicKey,
    updateAuthority: PublicKey,
    mint: PublicKey,
    mintAuthority: PublicKey
) {
    const layout = borsh.struct([
        borsh.vec(borsh.u8(), "name"),
        borsh.vec(borsh.u8(), "symbol"),
        borsh.vec(borsh.u8(), "uri"),
    ]);

    const keys: Array<AccountMeta> = [
        { pubkey: metadata,        isSigner: false, isWritable: true },
        { pubkey: updateAuthority, isSigner: false, isWritable: false },
        { pubkey: mint,            isSigner: false, isWritable: false },
        { pubkey: mintAuthority,   isSigner: true, isWritable: true },
    ];

    // sha256 spl_token_metadata_interface:initialize_account
    // d2e11ea258b84d8d

    // hex.decode(d2e11ea258b84d8d) = 210 225 30 162 88 184 77 141
    const identifier = Buffer.from([210, 225, 30, 162, 88, 184, 77, 141]);
    const buffer = Buffer.alloc(1000);

    const len = layout.encode(
        {
            name: Buffer.from("MyTokenName"),
            symbol: Buffer.from("TOKEN"),
            uri: Buffer.from("http://my.token")
        },
        buffer
    );

    const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len);
    const ix = new TransactionInstruction({ keys, programId: TOKEN_2022_PROGRAM_ID, data });

    return ix;
}