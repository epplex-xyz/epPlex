
import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";
import { serialize } from 'borsh';
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
export function createMetadataInstruction(
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

    // Figured out this part based on bbs4CMz9JL3JBW7wH7wq4q3sbBRV3PVAK5U7iKgFKFN
    // https://explorer.solana.com/tx/4issGVsr88SfW2K191VXZiKe5Jdjxm4dVDyqism6SqTaZayfhUfGD6RQv8SnabZTta9QqiRVgYxQ5BeCPnXKx2UA?cluster=devnet
    // and solana-program-library/token-metadata/interface/serc/instruction initialize
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


// doh, the reason i couldnt do anything from the terminal was because, I set the update auth from the example
// and not my own
// solana-program-library/token/cli/src/main CommandName::UpdateMetadata
/**
 * TODO: add
 *
 * @param metadata
 * @param updateAuthority
 *
 * @return Instruction to add to a transaction
 */
export function updateMetadataInstruction(
    metadata: PublicKey,
    updateAuthority: PublicKey,
) {

    const layout = borsh.struct([
        borsh.u8("enum"),
        borsh.vec(borsh.u8(), "field"),
        borsh.vec(borsh.u8(), "value"),
    ]);

    // Figured out this part based on
    // solana-program-library/token-metadata/interface/serc/instruction update_field
    const keys: Array<AccountMeta> = [
        { pubkey: metadata,        isSigner: false, isWritable: true },
        { pubkey: updateAuthority, isSigner: true, isWritable: false },
    ];

    // sha256 spl_token_metadata_interface:updating_field
    // dde9312db5cadcc8

    // {key: {}}
    // hex.decode(dde9312db5cadcc8) = 221 233 49 45 181 202 220 200
    const identifier = Buffer.from([221, 233, 49, 45, 181, 202, 220, 200]);
    const buffer = Buffer.alloc(1000);

    const len = layout.encode(
        {
            // field: [new Uint8Array([3]), Buffer.from("destroyTimestamp")],
            enum: Buffer.from("3"),
            field: Buffer.from("destroyTimestamp"),
            value: Buffer.from("1700733093"), // Thu Nov 23 2023 09:51:33 GMT+0000
        },
        buffer
    );

    const data = Buffer.concat([identifier, buffer]).slice(0, 8 + len);
    const ix = new TransactionInstruction({ keys, programId: TOKEN_2022_PROGRAM_ID, data });

    return ix;
}