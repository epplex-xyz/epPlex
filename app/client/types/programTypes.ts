import { AnchorProvider, Program } from "@coral-xyz/anchor";
import EpplexCoreIdl from "../idl/epplex_core.json";
import { EpplexCore, IDL as CoreIdl } from "../idl/epplexCoreTypes";

import EpplexBurgerIdl from "../idl/epplex_burger.json";
import { EpplexBurger, IDL as BurgerIdl } from "../idl/epplexBurgerTypes";
import { PublicKey } from "@solana/web3.js";


/*
 * BURGER PROGRAM
 */
export type EpplexBurgerProgram = Program<EpplexBurger>;
export const BURGER_PROGRAM_ID = new PublicKey(EpplexBurgerIdl.metadata.address);
export const createBurgerProgram= (provider: AnchorProvider) => new Program(BurgerIdl, BURGER_PROGRAM_ID, provider);


/*
 * EPPLEX CORE
 */
export type EpplexCoreProgram = Program<EpplexCore>;
export const CORE_PROGRAM_ID = new PublicKey(EpplexCoreIdl.metadata.address);
export const createCoreProgram= (provider: AnchorProvider) => new Program(CoreIdl, CORE_PROGRAM_ID, provider);






// export enum ProgramType {
//     burger,
//     core
// }

// export const createProgram = (provider: AnchorProvider, programType: ProgramType) => {
//     switch (programType) {
//         case ProgramType.burger: {
//             return new Program(BurgerIdl, BURGER_PROGRAM_ID, provider);
//         }
//         case ProgramType.core: {
//             return new Program(CoreIdl, CORE_PROGRAM_ID, provider);
//         }
//
//         default: {
//             break;
//         }
//     }
// };