import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Wallet } from "@coral-xyz/anchor";
import { ShadowUploadResponse, ShdwDrive } from "@shadow-drive/sdk";

export class ShdwProgram {
    connection: Connection;

    signer: Keypair;

    drive: ShdwDrive;

    constructor(
        signer: Keypair,
        connection: Connection,
    ) {
        this.connection = connection;
        this.signer = signer;
        this.drive = new ShdwDrive(connection, new Wallet(signer));
    }

    async uploadPng(fileName: string, fileBuffer: Buffer ): Promise<string> {
        const drive = await this.drive.init();
        const upload = await drive.uploadFile(
            new PublicKey(process.env.SHDW_ACCOUNT as string),
            {
                name: fileName,
                file: Buffer.from(fileBuffer)
            }
        );
        return upload.finalized_locations[0];
    }

    async createStorageAccount(
        name: string,
        storage: string = "100MB",
    ): Promise<void> {
        console.log("Create storage account");
        const drive = await this.drive.init();
        const newAccount = await drive.createStorageAccount(name, storage);
    }
}