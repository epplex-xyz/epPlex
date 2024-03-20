#!/usr/bin/env node
/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable import/no-extraneous-dependencies */

/**
 * This script will
 *  - Build any anchor projects if missing
 *  - Grab anchor project IDs
 *  - Update project IDs in Anchor.toml and lib.rs
 */

import shell from "shelljs"
import path from "path"

const projectRoot = path.join(__dirname, "..");

function replaceId(programName: string, programDir: string) {
    // REPLACE PROGRAM IDS
    console.log(`Replace Program ID:    ${programName} ${programDir}`);
    shell.sed(
        "-i",
        // Replace devnet
        `${programName} = { git = \\"https://github.com/wen-community/wen-new-standard.git\\"`,
        // With local path
        `${programName} = { path = "../${programName}"`,
        path.join(projectRoot, "programs", programDir, "Cargo.toml")
    );
}

async function main() {
    shell.cd(projectRoot);

    replaceId("wen_new_standard", "epplex-burger")
    replaceId("wen_royalty_distribution", "epplex-burger")

    replaceId("wen_new_standard", "epplex-core")
    replaceId("wen_royalty_distribution", "epplex-core")
}

main()
    .then(() => {
        console.log("Executed successfully");
    })
    .catch((err) => {
        console.error(err);
    });
