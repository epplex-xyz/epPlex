import { Connection, Keypair } from "@solana/web3.js";
import { COMMITMENT } from "../client/constants";
import { ShdwProgram } from "../client/shdwProgram";

const signer = Keypair.fromSecretKey(new Uint8Array(JSON.parse(process.env.SIGNER as string)));
// TODO exposed helius key
const connection = new Connection("https://rpc.helius.xyz/?api-key=8648c266-e57e-4acf-9405-24477d80975a", COMMITMENT);
export const Drive = new ShdwProgram(signer, connection);