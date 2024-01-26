import { AnchorProvider, Program } from "@coral-xyz/anchor";
import EpplexCoreIdl from "../idl/epplex_core.json";
import { EpplexCore, IDL as CoreIdl } from "../idl/epplexCoreTypes";

import EpplexBurgerIdl from "../idl/epplex_burger.json";
import { EpplexBurger, IDL as BurgerIdl } from "../idl/epplexBurgerTypes";
import { PublicKey } from "@solana/web3.js";

export const SEED_BURGER_METADATA = Buffer.from(JSON.parse(
    EpplexBurgerIdl.constants.filter(obj => {
        return obj.name === "SEED_BURGER_METADATA";
    })[0].value
));

export const SEED_PROGRAM_DELEGATE = Buffer.from(JSON.parse(
    EpplexBurgerIdl.constants.filter(obj => {
        return obj.name === "SEED_PROGRAM_DELEGATE";
    })[0].value
));



/*
 * BURGER PROGRAM
 */
// epBuJysRKuFMMWTWoX6ZKPz5WTZWb98mDqn1emVj84n
export type EpplexBurgerProgram = Program<EpplexBurger>;
export const BURGER_PROGRAM_ID = new PublicKey(EpplexBurgerIdl.metadata.address);
export const createBurgerProgram= (provider: AnchorProvider) => new Program(BurgerIdl, BURGER_PROGRAM_ID, provider);


/*
 * EPPLEX CORE
 */
// epCoD6BqcNinLvKN3KkY55vk4Kxs3W1JTENs1xqWUTg
export type EpplexCoreProgram = Program<EpplexCore>;
export const CORE_PROGRAM_ID = new PublicKey(EpplexCoreIdl.metadata.address);
export const createCoreProgram= (provider: AnchorProvider) => new Program(CoreIdl, CORE_PROGRAM_ID, provider);



export const VAULT = new PublicKey("BuHRzpGi4t9ho8rtBNCKCRrPE26EG2CGsq3YiVCkhXr7");



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