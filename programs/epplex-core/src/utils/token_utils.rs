use crate::*;
use epplex_metadata::CreateMetadataParams;


 pub fn token_create_basic<'info> (
    mint: AccountInfo<'info>,
    program_delegate: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    rent_account: AccountInfo<'info>,
    token22_program: AccountInfo<'info>,
    additional_extensions: &[ExtensionType]
) -> Result<()> {

    // Initialise Mint Account
    init_mint_account(
        rent_account.to_account_info(),
        payer.to_account_info(),
        mint.to_account_info(),
        additional_extensions
    )?;

    // Add closing authority
    add_closing_authority(
        &mint,
        token22_program.key(),
        program_delegate.key(),
    )?;

    // Add permanent delegate
    add_permanent_delegate(
        &mint.to_account_info(),
        token22_program.key(),
        program_delegate.key()
    )?;
    
    Ok(())
}

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
    rent_account: &AccountInfo<'info>,
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

pub fn create_metadata_account<'info>(
    metadata_program: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    token_metadata: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    params: TokenCreateParams
) -> Result<()> {
        //calculate destroy timestamp
        let now = Clock::get().unwrap().unix_timestamp;
        let destroy_timestamp = now
            .checked_add(params.destroy_timestamp_offset)
            .ok_or(EphemeralityError::InvalidCalculation)
            .unwrap();

        //create metadata account
        let cpi_ctx = CpiContext::new(
            metadata_program,
            epplex_metadata::cpi::accounts::CreateMetadata {
                payer: payer,
                mint: mint,
                token_metadata: token_metadata,
                system_program: system_program
            }
        );

        let cpi_params = CreateMetadataParams {
            destroy_timestamp: destroy_timestamp,
            name: params.name,
            symbol: params.symbol,
            uri: params.uri
        };

        epplex_metadata::cpi::create_metadata(cpi_ctx, cpi_params)?;

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

pub fn add_group_pointer(
    token_program_id: Pubkey,
    mint_account: &AccountInfo,
    authority: Pubkey,
    group_address: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::extension::group_pointer::instruction::initialize(
        &token_program_id,
        &mint_account.key(),
        Some(authority),
        Some(group_address)
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn add_group_member_pointer(
    token_program_id: Pubkey,
    mint_account: &AccountInfo,
    authority: Pubkey,
    group_member_address: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::extension::group_member_pointer::instruction::initialize(
        &token_program_id,
        &mint_account.key(),
        Some(authority),
        Some(group_member_address)
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn init_mint_account<'info> (
    rent_account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    additional_extensions: &[ExtensionType]
) -> Result<()> {

    // standard extensions
    let mut extensions = vec![
        ExtensionType::PermanentDelegate,
        ExtensionType::MintCloseAuthority
    ];

    extensions.extend_from_slice(additional_extensions);

    // calculate extension sizes
    let extension_sizes = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions).unwrap();

    msg!(&extension_sizes.to_string());

    let rent = &Rent::from_account_info(&rent_account)?;
    // TODO need to have collectionConfig passed in

    // TODO: all NFTs should have same expiration date upon mint
    // maybe just save the now date and the destroytimeoffset

    // TODO: need to calculate this properly
    let ix = solana_program::system_instruction::create_account(
        &payer.key(),
        &mint.key(),
        rent.minimum_balance(extension_sizes),
        extension_sizes as u64,
        &spl_token_2022::id(),
    );

    let account_infos: Vec<AccountInfo> = vec![
        payer,
        mint
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn add_closing_authority(
    mint_account: &AccountInfo,
    program: Pubkey,
    program_delegate: Pubkey
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_mint_close_authority(
        &program,
        &mint_account.key(),
        Some(&program_delegate),
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn add_permanent_delegate(
    mint_account: &AccountInfo,
    program: Pubkey,
    program_delegate: Pubkey
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_permanent_delegate(
        &program,
        &mint_account.key(),
        &program_delegate,
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info()
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
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