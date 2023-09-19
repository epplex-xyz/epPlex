import { IdlAccounts, Program, ProgramAccount, AnchorProvider } from "@coral-xyz/anchor";
import idl from '../idl/ephemerality.json';
import { Ephemerality } from "../idl/ephemeralityTypes";
import { IDL } from "../idl/ephemeralityTypes";
import { PublicKey } from "@solana/web3.js";


export type EphemeralityProgram = Program<Ephemerality>;
export const PROGRAM_ID = new PublicKey(idl.metadata.address);

export const createProgram = (provider: AnchorProvider) => new Program(IDL, PROGRAM_ID, provider);