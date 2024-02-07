import {AnchorProvider, BN} from "@coral-xyz/anchor";
import {sendAndConfirmRawTransaction} from "../app/utils/solana";
import {BurgerProgram} from "../app/client/burgerProgram";
import {Keypair, PublicKey} from "@solana/web3.js";
import {CoreProgram} from "../app/client/coreProgram";
import {collectionConfig} from "./pda";

export const mintTokenIntoCollection = async (provider: AnchorProvider, burgerProgram: BurgerProgram,coreProgram: CoreProgram, collectionId: BN, destroyTimestamp: string) => {

    const [collectionConfigAddress, _bump] = PublicKey.findProgramAddressSync(
        [Buffer.from("CONFIG"),
            collectionId.toArrayLike(Buffer, "le", 8)],
        coreProgram.program.programId
    )
    const collectionConfigData = await coreProgram.program.account.collectionConfig.fetch(collectionConfigAddress);
    console.log("collectionId", collectionId.toString());
    console.log("mintCount", collectionConfigData.mintCount.toString());


    const [mint, _] = PublicKey.findProgramAddressSync(
        [Buffer.from("MINT"),
            collectionId.toArrayLike(Buffer, "le", 8),
            collectionConfigData.mintCount.toArrayLike(Buffer, "le", 8)
            ],
        coreProgram.program.programId
    )
    // Mint a token into the collection
    const tx = await burgerProgram.createCollectionMintTx(
        destroyTimestamp,
        collectionId,
        mint,
    )
    console.log("rpc", provider.connection.rpcEndpoint);
    await sendAndConfirmRawTransaction(
        provider.connection,
        tx,
        provider.publicKey,
        provider.wallet,
        []
    );
    return mint;
}