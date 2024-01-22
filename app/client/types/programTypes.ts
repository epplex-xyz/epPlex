import { AnchorProvider, Program } from "@coral-xyz/anchor";
import EpplexCoreIdl from "../idl/epplex_core.json";
import { EpplexCore, IDL as CoreIdl } from "../idl/epplexCoreTypes";

import EpplexBurgerIdl from "../idl/epplex_burger.json";
import { EpplexBurger, IDL as BurgerIdl } from "../idl/epplexBurgerTypes";
import { PublicKey } from "@solana/web3.js";


export type EpplexBurgerProgram = Program<EpplexBurger>;
export const PROGRAM_ID = new PublicKey(EpplexBurgerIdl.metadata.address);

export const createProgram = (provider: AnchorProvider) => new Program(BurgerIdl, PROGRAM_ID, provider);