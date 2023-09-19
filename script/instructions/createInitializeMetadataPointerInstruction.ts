import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

// Invalid instruction at first
// https://explorer.solana.com/tx/4smGbr2G25BYKtccAzrH38hbM1heT9xjSEDKWbEFqYtsWqU9Be6BnUzFRZVL11frJWgxqwTWvBgWWUqhhmzBqsFS?cluster=devnet#ix-5


// Testing through terminal for bbs4CMz9JL3JBW7wH7wq4q3sbBRV3PVAK5U7iKgFKFN
// Metadatapointer
// https://explorer.solana.com/tx/NLs3tdB4NZBYUfZwPEuVpLtdMj5tmFCZU1ZcDAvGsLv6SJpZQgrTCeq8qDGceUAPwmQ4G97wtBNtXwEQk719eMK?cluster=devnet

// TokenMetadata initialization
// https://explorer.solana.com/tx/4issGVsr88SfW2K191VXZiKe5Jdjxm4dVDyqism6SqTaZayfhUfGD6RQv8SnabZTta9QqiRVgYxQ5BeCPnXKx2UA?cluster=devnet


// Found from program-2022/src/instruction.rs
// Actually being processed in program-2022/src/processor.rs
enum Instruction {
    MetadataPointer = 39
}

enum MetadataInstruction {
    Initialize = 0,
    Update = 1
}

// This requires another instruction since it is nested within the token program within a folder
export interface MetadataPointerInstructionData {
    instruction: Instruction.MetadataPointer;
    metadataPointerInstruction: MetadataInstruction.Initialize
    authority: PublicKey;
    metadataAddress: PublicKey;
}

/** TODO: docs */
export const initialzeMetadataPointerInstructionData = borsh.struct<MetadataPointerInstructionData>([
    borsh.u8('instruction'),
    borsh.u8('metadataPointerInstruction'),
    borsh.publicKey('authority'),
    borsh.publicKey('metadataAddress'),
]);


/**
 * Construct an InitializePermanentDelegate instruction
 *
 * @param mint               Token mint account
 * @param authority  Authority that may sign for `Transfer`s and `Burn`s on any account
 * @param metadataAddress  Authority that may sign for `Transfer`s and `Burn`s on any account
 * @param programId          SPL Token program account
 *
 * @return Instruction to add to a transaction
 */
export function createInitializeMetadataPointerInstruction(
    mint: PublicKey,
    authority: PublicKey | null,
    metadataAddress: PublicKey | null,
    programId: PublicKey
): TransactionInstruction {
    const keys = [{ pubkey: mint, isSigner: true, isWritable: true }];

    const data = Buffer.alloc(initialzeMetadataPointerInstructionData.span);
    initialzeMetadataPointerInstructionData.encode(
        {
            instruction: Instruction.MetadataPointer,
            metadataPointerInstruction: MetadataInstruction.Initialize,
            authority: authority || new PublicKey(0),
            metadataAddress: metadataAddress || new PublicKey(0),
        },
        data
    );

    return new TransactionInstruction({ keys, programId, data });
}

