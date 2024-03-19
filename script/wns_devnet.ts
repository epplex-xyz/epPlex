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
// import {spawn} from "child_process"
import fs from "fs"
import path from "path"

const projectRoot = path.join(__dirname, "..");
// const targetDir = path.join(projectRoot, "target");


function replaceId(programName: string, programDir: string) {
    // REPLACE PROGRAM IDS
    console.log(`Replace Program ID:    ${programName}`);
    shell.sed(
        "-i",
        `# ${programName}`,
        `${programName}`,
        path.join(projectRoot, "programs", programDir, "Cargo.toml")
    );

    shell.sed(
        "-i",
        `{programName}`,
        `# ${programName}`,
        path.join(projectRoot, "programs", programDir, "Cargo.toml")
    );

}
async function main() {
    shell.cd(projectRoot);

    // if (!shell.which("solana")) {
    //     shell.echo(
    //         "Sorry, this script requires 'solana' to be installed in your $PATH"
    //     );
    //     shell.exit(1);
    // }

    // if (!shell.which("anchor")) {
    //     shell.echo(
    //         "Sorry, this script requires 'anchor' to be installed in your $PATH"
    //     );
    //     shell.exit(1);
    // }

    // if (!fs.existsSync(path.join(targetDir, "deploy"))) {
    //     shell.echo("Missing program deploy keypairs, building projects");
    //     const anchorBuildSpawn = spawn("anchor", ["build"]);
    //     anchorBuildSpawn.stdout.on("data", function (msg) {
    //         console.log(msg.toString());
    //     });
    //     await new Promise((resolve) => {
    //         anchorBuildSpawn.on("close", resolve);
    //     });
    // }

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
