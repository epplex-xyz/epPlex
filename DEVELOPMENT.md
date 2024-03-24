# Local development setup

## Environment

For reproducibility, these are the current versions of the developer tooling used

`anchor v0.29.0`

`solana-cli suite v1.16.27`

`node v20.8.1`

`yarn 1.22.19`

- clone the protocol and the SDK repo into the same directory

```bash
git clone https://github.com/epplex-xyz/epPlex.git

git clone https://github.com/epplex-xyz/sdk.git
```

### program workflow

- Install Typescript dependencies

```bash
yarn install
```

- Build the anchor program

```bash
anchor build
```

- Update program keypairs in the `target/deploy` directory.

    **NOTE:** make sure the files end in `-keypair.json` ****


```bash
cp epplex_burger-keypair.json epplex_core-keypair.json epplex_shared-keypair.json /home/jimii/Documents/rustcode/epPlex/target/deploy
```

Running `anchor build` and `anchor keys list` should result in something like this

```bash
epplex_core: LepCn3tW66Fh7CGsJ7qjQaontU7SvEoURnxkEY78j1j
epplex_burger: LepByYNXCXLAQicahdRvvxBD45SMNHJgoNsAUDLyG1N
epplex_shared: LepS8gH3rVDRdAy5X9xGE8VcnGnFKmkpfm21Paiqq3i
```

- Update the `[programs.localnet]` in `Anchor.toml` with the values obtained above, if they are different

```toml
[programs.localnet]
epplex_core = "LepCn3tW66Fh7CGsJ7qjQaontU7SvEoURnxkEY78j1j"
epplex_burger = "LepByYNXCXLAQicahdRvvxBD45SMNHJgoNsAUDLyG1N"
epplex_shared = "LepS8gH3rVDRdAy5X9xGE8VcnGnFKmkpfm21Paiqq3i"
```

- Update program IDs for each program in the `id.rs`  file, if different.
- Run the final `anchor build` to sync the changes.
- In a new terminal window start the `solana-test-validator`
**P.S:** To clear the history, remove the `test-ledger` directory.
- Change the Solana config to point to `localnet`

```bash
solana config set --url localhost
```

- We will the `epplex_deploy_auth.json` -*LAutdv6yTevtPKCpfH21BwkCZhZSfov561GKYyRp5pC*  as the program authority.
Copy this keypair file into the `target/deploy` directory a

```bash
cp epplex_deploy_auth.json /home/jimii/Documents/rustcode/epPlex/target/deploy
```

- Airdrop yourself some sol

```bash
solana airdrop 50 LAutdv6yTevtPKCpfH21BwkCZhZSfov561GKYyRp5pC
```

- Deploy the program

```bash
anchor deploy
```

### SDK and testing workflow

Change directory into the SDK repo

- Install the dependencies

```bash
yarn install
```

- Update IDL and types if you made changes in the program repo

```bash
yarn run update-types-all && yarn run update-idl-all
```

- Cope the `epplex_PAYER_ADMIN.json` *LAdmTEtom7qm3ZmchsrqSkZhPdmZaex7oXCamuMHs9F keypair file* into the `.local_keys` directory
**If the directory does not exist create it first**.

```bash
cp epplex_PAYER_ADMIN.json /home/jimii/Documents/crew/sdk/.local_keys
```

- Airdrop yourself some sol

```bash
solana airdrop 50 *LAdmTEtom7qm3ZmchsrqSkZhPdmZaex7oXCamuMHs9F*
```

- Run a test case

```bash
yarn run test-individual
```