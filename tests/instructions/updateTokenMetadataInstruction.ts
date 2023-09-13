import { TransactionInstruction, PublicKey } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

enum Instruction {
    MetadataPointer = 39
}

enum MetadataInstruction {
    Initialize = 0,
    Update = 1
}

// This requires another instruction since it is nested within the token program within a folder
export interface UpdateTokenMetadataData {
    instruction: Instruction.MetadataPointer;
    metadataPointerInstruction: MetadataInstruction.Update
    authority: PublicKey;
    metadataAddress: PublicKey;
}

/** TODO: docs */
export const updateTokenMetadataInstructionData = borsh.struct<UpdateTokenMetadataData>([
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
export function updateTokenMetadataInstruction (
    mint: PublicKey,
    authority: PublicKey | null,
    metadataAddress: PublicKey | null,
    programId: PublicKey
): TransactionInstruction {
    const keys = [{ pubkey: mint, isSigner: true, isWritable: true }];

    const data = Buffer.alloc(updateTokenMetadataInstructionData.span);
    updateTokenMetadataInstructionData.encode(
        {
            instruction: Instruction.MetadataPointer,
            metadataPointerInstruction: MetadataInstruction.Update,
            authority: authority || new PublicKey(0),
            metadataAddress: metadataAddress || new PublicKey(0),
        },
        data
    );

    return new TransactionInstruction({ keys, programId, data });
}

