use crate::*;
use solana_program::{system_instruction, instruction::Instruction};

pub fn burn_token<'info>(
    mint_account: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    program: Pubkey,
    authority: &Account<'info, ProgramDelegate>
) -> Result<()> {
    let ix = spl_token_2022::instruction::burn(
        &program,
        &token_account.key(),
        &mint_account.key(),
        &authority.key(),
        &[],
        1
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        mint_account.to_account_info(),
        authority.to_account_info(),
    ];

    let seeds = &[SEED_PROGRAM_DELEGATE, &[authority.bump]];
    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&seeds[..]]
    )?;

    Ok(())
}

pub fn close_mint<'info>(
    program: Pubkey,
    token_account: &AccountInfo<'info>,
    destination_account: &AccountInfo<'info>,
    owner: &Account<'info, ProgramDelegate>
) -> Result<()> {
    let ix = spl_token_2022::instruction::close_account(
        &program,
        &token_account.key(),
        &destination_account.key(),
        &owner.key(),
        &[]
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        destination_account.to_account_info(),
        owner.to_account_info(),
    ];

    let seeds = &[SEED_PROGRAM_DELEGATE, &[owner.bump]];
    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&seeds[..]]
    )?;

    Ok(())
}

pub fn initialize_mint<'info>(
    mint_account: &AccountInfo<'info>,
    rent_account: &Sysvar<'info, Rent>,
    program: &Pubkey,
    mint_auth: &Pubkey,
    freeze_auth: &Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_mint(
        &program,
        &mint_account.key(),
        &mint_auth, // this could be different I guess
        Some(&freeze_auth), // free auth just set to payer as well
        0, // NFTs have 0 decimals
    )?;

    // TODO: why are these cloned in the token22 source code
    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
        rent_account.to_account_info()
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())

}

// actually does anchor spl_token have the src/extension/metadatapointer?
pub fn add_metadata_pointer(
    token_program_id: Pubkey,
    mint_account: &AccountInfo,
    authority: Pubkey,
    metadata_address: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        &token_program_id,
        &mint_account.key(),
        Some(authority),
        Some(metadata_address)
    )?;

    // TODO: frontend instruction passed in 4 accountinfos and I think 1 is enough
    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

// actually does anchor spl_token have the src/extension/metadatapointer?
pub fn add_token_metadata<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::initialize(
        &program_id,
        &metadata.key(),
        &update_authority.key(),
        &mint.key(),
        &mint_authority.key(),
        name,
        symbol,
        uri
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
        mint.to_account_info(),
        mint_authority.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn update_token_metadata<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        &program_id,
        &metadata.key(),
        &update_authority.key(),
        field,
        value
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

// Probably system program already has this callable, I just copied from there
pub fn transfer_sol<'info>(
    program_id: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    let ix = system_instruction::transfer(
        &from.key(),
        &to.key(),
        amount,
    );

    let account_infos: Vec<AccountInfo> = vec![
        from.to_account_info(),
        to.to_account_info(),
        program_id.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}


pub fn create_collection_instruction(
    payer: &Pubkey, // The account paying for the transaction
    program_id: &Pubkey, // The Program ID of your CollectionCreate program
    mint: &Pubkey, // The mint account
    program_delegate: &Pubkey, // The program delegate account
    collection_config: &Pubkey, // The collection config account
    token22_program_id: &Pubkey, // The Token22 program ID
    params: CollectionCreateParams, // The parameters for collection creation
    system_program_id: &Pubkey, // The System Program ID
) -> Result<Instruction> {
    // Define the accounts that will be passed to the instruction
    let accounts = vec![
        AccountMeta::new(*mint, false),
        AccountMeta::new_readonly(*program_delegate, false),
        AccountMeta::new(*collection_config, false),
        AccountMeta::new(*payer, true), // Signer account must be marked as a signer
        AccountMeta::new_readonly(*token22_program_id, false),
        AccountMeta::new_readonly(*system_program_id, false),
    ];

    // Serialize the instruction data
    let instruction_data = CollectionCreateParams {
        authority: params.authority,
        renewal_price: params.renewal_price,
        standard_duration: params.standard_duration,
        grace_period: params.grace_period,
        treasury: params.treasury,
        collection_size: params.collection_size,
        collection_name: params.collection_name,
        collection_symbol: params.collection_symbol
    };
    let data = instruction_data.try_to_vec()?; // Using Borsh to serialize

    // Create the instruction
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

// Fails here
// https://explorer.solana.com/tx/33rZroF4LnJ8Buu3fnpeE7gHRWjBcJwecmrByMuC7CKxxJzpT9oqFge99T4zqwnSDkUAttUeN4E4ADa6F8wVnYQu?cluster=devnet

// /// CHECK: account checked in CPI
// #[account(mut)]
// metadata: UncheckedAccount<'info>,
//
// /// CHECK: account checked in CPI
// #[account(address = mpl_token_metadata::id())]
// token_metadata_program: UncheckedAccount<'info>,

// pub fn create_mpl_token_metadata<'info>(
//     program_id: &AccountInfo<'info>,
//     from: &AccountInfo<'info>,
//     to: &AccountInfo<'info>,
//     amount: u64
// ) -> Result<()> {
//     let metadata_infos: Vec<AccountInfo> = vec![
//         ctx.accounts.metadata.to_account_info(),
//         ctx.accounts.mint.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.payer.to_account_info(),
//         ctx.accounts.token_metadata_program.to_account_info(),
//     ];
//
//     let creators = vec![
//         mpl_token_metadata::state::Creator {
//             address: ctx.accounts.payer.key(),
//             verified: false,
//             share: 100,
//         },
//     ];
//
//     // This cannot work since Metaplex only checks for owner of Standard Token program, not Token22
//     // mpl_token_metadata/src/utils/metadata/Process_create_metadata_accounts_logic
//     // MetadataInstruction::CreateMetadataAccountV3
//     // mpl_token_metadata/src/processor/mod/process_legacy_instruction/process_create_metadata_accounts_v3
//     let ix = create_metadata_accounts_v3(
//         ctx.accounts.token_metadata_program.key(),
//         ctx.accounts.metadata.key(),
//         ctx.accounts.mint.key(),
//         ctx.accounts.payer.key(),
//         ctx.accounts.payer.key(),
//         ctx.accounts.payer.key(),
//         "MyTokenName".to_string(),
//         "TOKEN".to_string(),
//         "https://arweave.net/nVRvZDaOk5YAdr4ZBEeMjOVhynuv8P3vywvuN5sYSPo".to_string(),
//         Some(creators),
//         500,
//         false,
//         true,
//         None,
//         None,
//         None,
//     );
//
//     solana_program::program::invoke(
//         &ix,
//         metadata_infos.as_slice(),
//     )?;
//
//     Ok(())
// }