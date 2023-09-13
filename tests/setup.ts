import {loadOrGenerateKeypair, savePublicKeyToFile} from "./utils/helpers";
import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey,
    TransactionInstruction,
    AccountMeta
} from "@solana/web3.js";

import {
    ExtensionType,
    createInitializeMintInstruction,
    createInitializePermanentDelegateInstruction,
    mintTo,
    createAccount,
    getMintLen,
    transferChecked,
    TOKEN_2022_PROGRAM_ID,
    getOrCreateAssociatedTokenAccount,
    createBurnInstruction,
    burnInstructionData,
    TokenInstruction,
    createInitializeMintCloseAuthorityInstruction,
    createCloseAccountInstruction,
    MintLayout,
    AccountLayout
} from "@solana/spl-token";
import * as borsh from "@coral-xyz/borsh";
import {metadataInstruction} from "./createInitializeTokenMetadataInstruction";
import {createInitializeMetadataPointerInstruction} from "./createInitializeMetadataPointerInstruction";

const rpc = "https://api.devnet.solana.com";
const connection = new Connection(rpc, "confirmed");

// Old mint https://solscan.io/token/3s792R18rLLvrGmFYk373jVSML7xh6SvsW5ZiXTxTk3Y?cluster=devnet, only has authority field

// Old mint with permanentDelegate/closing auth Vzpbwg4jYWAoLfXesLW4Ni5itcyS6h5PB1K2m3S5CaC


// it is possible to read the full account data based on `solana account <pubkey>`

const layout = borsh.struct([
    borsh.publicKey("updateAuthority"),
    borsh.publicKey("mint"),
    borsh.array(borsh.u8(), 11, "name"),
    borsh.array(borsh.u8(), 4, "symbol"),
    borsh.array(borsh.u8(), 20, "uri"),
]);


const TOKEN_METADATA_SIZE = layout.span + 500;


async function setup() {

    // Collection auth and treeCreator
    const payer = loadOrGenerateKeypair("payer");
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");
    // const permanentDelegate = new PublicKey("2N6aJDX1TNs6RKkPsuufbAe4JjRAZPs1iLPcEUL4DX4z");

    // const airdropSignature = await connection.requestAirdrop(payer.publicKey, 2 * LAMPORTS_PER_SOL);
    // await connection.confirmTransaction({ signature: airdropSignature, ...(await connection.getLatestBlockhash()) });


    const extensions = [ExtensionType.MintCloseAuthority, ExtensionType.PermanentDelegate];
    // const mintLen = getMintLen(extensions) + TOKEN_METADATA_SIZE;
    const mintLen = getMintLen(extensions) + (64 + 2 + 2);
    // console.log("length", getMintLen(extensions));
    // console.log("length", mintLen);
    const decimals = 0;
    const mintLamports = await connection.getMinimumBalanceForRentExemption(mintLen);


    const mintTransaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint,
            space: mintLen,
            lamports: mintLamports,
            programId: TOKEN_2022_PROGRAM_ID,
        }),

        // How do I decode the metadata for my own viewing?

        createInitializeMintCloseAuthorityInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        createInitializePermanentDelegateInstruction(mint, permanentDelegate.publicKey, TOKEN_2022_PROGRAM_ID),
        // TODO
        // metadatapointer should happen after Account creation, before mint initialization
        // Error because account sizing is wrong. Proper space has been allocated to the above two, but not the metadatapointer
        // If I put this as the first ix, it succeeds

        // So there is a difference between the span/sizing of an instruction and the config/account/state size.
        // so basically had to account for how it does the computations on sizing. All I needed was two pubkey sizing (32*2=64)
        // in addition to 2 + 2 for the default computational aspects SIZE+LENGTH
        createInitializeMetadataPointerInstruction(mint, permanentDelegate.publicKey, mint, TOKEN_2022_PROGRAM_ID),

        createInitializeMintInstruction(mint, decimals, mintAuthority.publicKey, null, TOKEN_2022_PROGRAM_ID),

        // These two are tied together
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: mint,
            lamports: BigInt(1000000),
        }),
        metadataInstruction(mint, permanentDelegate.publicKey, mint, mintAuthority.publicKey),
    );
    const txId = await sendAndConfirmTransaction(connection, mintTransaction, [payer, mintKeypair, mintAuthority], {skipPreflight: true});
    console.log("tx", txId);

    savePublicKeyToFile("mintPubkey", mint);
}

async function mint() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");

    // Get the token account of the toWallet address, and if it does not exist, create it
    const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey,
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("token", fromTokenAccount.address.toString());

    // Mint 1 new token to the "fromTokenAccount" account we just created
    let signature = await mintTo(
        connection,
        payer,
        mint,
        fromTokenAccount.address,
        mintAuthority,
        1,
        [],
        undefined,
        TOKEN_2022_PROGRAM_ID
    );

    console.log("tx", signature);
}


async function burn() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");


    // Get the token account of the toWallet address, and if it does not exist, create it
    const account = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey,
        undefined,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
    );


    const keys: AccountMeta[] = [
        { pubkey: account.address, isSigner: false, isWritable: true },
        { pubkey: mint, isSigner: false, isWritable: true },
        { pubkey: permanentDelegate.publicKey, isSigner: true, isWritable: true }
    ];

    const data = Buffer.alloc(burnInstructionData.span);
    burnInstructionData.encode(
        {
            instruction: TokenInstruction.Burn,
            amount: BigInt(1),
        },
        data
    );

    const ix = new TransactionInstruction({ keys, programId: TOKEN_2022_PROGRAM_ID, data });

    const transaction = new Transaction().add(
        createBurnInstruction(account.address, mint, permanentDelegate.publicKey, 1, [], TOKEN_2022_PROGRAM_ID),
        createCloseAccountInstruction(mint, payer.publicKey, permanentDelegate.publicKey, [], TOKEN_2022_PROGRAM_ID)
    );

    try {
        const tx = await sendAndConfirmTransaction(connection, transaction, [permanentDelegate]);
        console.log("tx", tx);
    } catch (e) {
        console.log("err", e);
    }
}

async function test() {
    const payer = loadOrGenerateKeypair("payer");
    console.log("payer", payer.publicKey.toString());
    const mintKeypair = loadOrGenerateKeypair("mint");
    const mint = mintKeypair.publicKey;
    console.log("mint", mint.toString());
    const mintAuthority = loadOrGenerateKeypair("mintAuth");
    const permanentDelegate = loadOrGenerateKeypair("permDelegate");


    const transaction = new Transaction().add(
        metadataInstruction(mint, permanentDelegate.publicKey, mint, mintAuthority.publicKey)
    );

    try {
        const tx = await sendAndConfirmTransaction(connection, transaction, [payer, mintAuthority], {skipPreflight: true});
        console.log("tx", tx);
    } catch (e) {
        console.log("err", e);
    }

}
// https://github.com/GoogleChromeLabs/jsbi/issues/30
function serialize(data) {
    return JSON.parse(JSON.stringify(data, (key, value) =>
        typeof value === 'bigint'
            ? value.toString()
            : value // return everything else unchanged
    ));
}

// export const MintLayout = struct<RawMint>([
//     u32('mintAuthorityOption'),
//     publicKey('mintAuthority'),
//     u64('supply'),
//     u8('decimals'),
//     bool('isInitialized'),
//     u32('freezeAuthorityOption'),
//     publicKey('freezeAuthority'),
// ]);

// decoded {
//     mintAuthorityOption: 1,
//     mintAuthority: 'CyNhPdPhFiRtarKKt29ETmPr8DSigNvgFmQWBdvcqynB',
//     supply: '0',
//     decimals: 0,
//     isInitialized: true,
//     freezeAuthorityOption: 0,
//     freezeAuthority: '11111111111111111111111111111111'
//   }
async function accountInfo() {
    // const layout = borsh.struct([
    //     borsh.publicKey("mintAuthority"),
    //     borsh.u64("supply"),
    //     borsh.u8("decimals"),
    //     borsh.bool("isInitialized"),
    //     borsh.publicKey("freezeAuthority"),
    // ]);

    const info = await connection.getAccountInfo(new PublicKey("8MBcTD24nCZeN3f73RNFCGW5HcD4C3y62VwjvLz8xpjr"));

    const decoded = MintLayout.decode(info.data);
    // const decoded = AccountLayout.decode(info.data.slice(8));
    console.log("decoded", serialize(decoded));

}
// accountInfo();

// setup();
// mint();
// test();
// burn();
// console.log("size", TOKEN_METADATA_SIZE);





// 0000:   01 00 00 00  b1 e1 9d 19  cb e9 58 bc  ef 85 7d 77   ..........X...}w
// 0010:   07 0b 9e 00  fc 43 77 2c  e0 37 6c 1c  a1 d3 44 94   .....Cw,.7l...D.
// 0020:   d3 c0 81 ec  00 00 00 00  00 00 00 00  00 01 00 00   ................
// 0030:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0040:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0050:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0060:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0070:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0080:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 0090:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
// 00a0:   00 00 00 00  00 01 03 00  20 00 80 81  06 07 a3 00   ........ .......
// 00b0:   32 0e ff d6  5c 94 59 6a  80 0c 09 2f  6f 17 b9 3a   2...\.Yj.../o..:
// 00c0:   55 6c 21 24  af cb 0c 31  36 66 0c 00  20 00 80 81   Ul!$...16f.. ...
// 00d0:   06 07 a3 00  32 0e ff d6  5c 94 59 6a  80 0c 09 2f   ....2...\.Yj.../
// 00e0:   6f 17 b9 3a  55 6c 21 24  af cb 0c 31  36 66 12 00   o..:Ul!$...16f..
// 00f0:   40 00 80 81  06 07 a3 00  32 0e ff d6  5c 94 59 6a   @.......2...\.Yj
// 0100:   80 0c 09 2f  6f 17 b9 3a  55 6c 21 24  af cb 0c 31   .../o..:Ul!$...1
// 0110:   36 66 6d 2d  6c 3e b5 fd  9d fb 3e c5  cb d1 19 91   6fm-l>....>.....
// 0120:   e2 0b 8f 8c  4c 32 57 2a  ee 56 e5 e2  5e b0 2e 78   ....L2W*.V..^..x
// 0130:   8f 39 13 00  6f 00 80 81  06 07 a3 00  32 0e ff d6   .9..o.......2...
// 0140:   5c 94 59 6a  80 0c 09 2f  6f 17 b9 3a  55 6c 21 24   \.Yj.../o..:Ul!$
// 0150:   af cb 0c 31  36 66 6d 2d  6c 3e b5 fd  9d fb 3e c5   ...16fm-l>....>.
// 0160:   cb d1 19 91  e2 0b 8f 8c  4c 32 57 2a  ee 56 e5 e2   ........L2W*.V..
// 0170:   5e b0 2e 78  8f 39 0b 00  00 00 4d 79  54 6f 6b 65   ^..x.9....MyToke
// 0180:   6e 4e 61 6d  65 05 00 00  00 54 4f 4b  45 4e 0f 00   nName....TOKEN..
// 0190:   00 00 68 74  74 70 3a 2f  2f 6d 79 2e  74 6f 6b 65   ..http://my.toke
// 01a0:   6e 00 00 00  00

// 26 rows X 16 columns + 5 = 421

// convert bytes to decimal representation
// b1e19d19cbe958bcef857d77070b9e00fc43772ce0376c1ca1d34494d3c081ec
const a = new Uint8Array([ 177, 225, 157, 25, 203, 233, 88, 188, 239, 133, 125, 119, 7, 11, 158, 0, 252, 67, 119, 44, 224, 55, 108, 28, 161, 211, 68, 148, 211, 192, 129, 236
]);
const pub = new PublicKey(a);
console.log(pub.toString());