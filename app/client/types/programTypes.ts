import { AnchorProvider, Program } from "@coral-xyz/anchor";
import EpplexCoreIdl from "../idl/epplex_core.json";
import { EpplexCore, IDL } from "../idl/epplexCoreTypes";
import { PublicKey } from "@solana/web3.js";


export type EpplexCoreProgram = Program<EpplexCore>;
export const PROGRAM_ID = new PublicKey(EpplexCoreIdl.metadata.address);

export const createProgram = (provider: AnchorProvider) => new Program(IDL, PROGRAM_ID, provider);