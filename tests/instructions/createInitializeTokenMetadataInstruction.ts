
import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import {
    TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";

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


// https://explorer.solana.com/tx/4issGVsr88SfW2K191VXZiKe5Jdjxm4dVDyqism6SqTaZayfhUfGD6RQv8SnabZTta9QqiRVgYxQ5BeCPnXKx2UA?cluster=devnet

// first 8 bytes is the instruction name, then comes 0b 00 00 00
// , which is 11 and denotes the length of the next part MyTokenName
// 05 00 00 00, denotes TOKEN
// and so on

// d2 e1 1e a2 58 b8 4d 8d 0b 00 00 00 4d 79 54 6f 
// 6b 65 6e 4e 61 6d 65 05 00 00 00 54 4f 4b 45 4e 
// 0f 00 00 00 68 74 74 70 3a 2f 2f 6d 79 2e 74 6f 
// 6b 65 6e 

// MyTokenName
// 4d 79 54 6f 6b 65 6e 4e 61 6d 65

// TOKEN
// 54 4f 4b 45 4e

// http://my.token
// 68 74 74 70 3a 2f 2f 6d 79 2e 74 6f 6b 65 6e