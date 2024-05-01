use crate::*;
use spl_token_metadata_interface::state::TokenMetadata;

pub fn initialize_mint<'info>(
    mint_account: &AccountInfo<'info>,
    rent_account: &AccountInfo<'info>,
    program: &Pubkey,
    mint_auth: &Pubkey,
    freeze_auth: &Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_mint(
        // &program,
        program,
        &mint_account.key(),
        // &mint_auth,
        mint_auth, // this could be different I guess
        // Some(&freeze_auth)
        Some(freeze_auth), // free auth just set to payer as well
        0,                 // NFTs have 0 decimals
    )?;

    // TODO: why are these cloned in the token22 source code
    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
        rent_account.to_account_info(),
    ];

    msg!("Accounts: {:?}", account_infos);

    solana_program::program::invoke(&ix, &account_infos[..])?;
    Ok(())
}

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
        Some(metadata_address),
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}

// actually does anchor spl_token have the src/extension/metadatapointer?
#[allow(clippy::too_many_arguments)]
pub fn initialize_token_metadata<'info>(
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
        program_id,
        // &program_id,
        &metadata.key(),
        &update_authority.key(),
        &mint.key(),
        &mint_authority.key(),
        name,
        symbol,
        uri,
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
        mint.to_account_info(),
        mint_authority.to_account_info(),
    ];

    solana_program::program::invoke(&ix, &account_infos[..])?;

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
        Some(group_address),
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

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
        Some(group_member_address),
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}

// TODO this function needs to be similar to create_token_2022_and_metadata in LibrePlex
pub fn init_mint_account<'info>(
    payer: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    rent_account: AccountInfo<'info>,
    extensions: &[ExtensionType],
    token_metadata: TokenMetadata,
    collection_id: u64,
    mint_count: u64,
) -> Result<()> {
    // Calculate extension sizes
    // let base_size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions).unwrap();
    let base_size =
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(extensions)
            .unwrap();
    let extension_extra_space = token_metadata.tlv_size_of().unwrap();

    let rent = &Rent::from_account_info(&rent_account)?;
    let lamports = rent.minimum_balance(base_size + extension_extra_space);
    let ix = solana_program::system_instruction::create_account(
        &payer.key(),
        &mint.key(),
        lamports,
        (base_size).try_into().unwrap(),
        &spl_token_2022::id(),
    );

    let account_infos: Vec<AccountInfo> = vec![payer, mint.clone()];
    let (expected_mint_account, bump) = Pubkey::find_program_address(
        &[
            SEED_MINT,
            collection_id.to_le_bytes().as_ref(),
            mint_count.to_le_bytes().as_ref(),
        ],
        &ID,
    );
    require_keys_eq!(expected_mint_account, mint.key());

    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&[
            SEED_MINT,
            collection_id.to_le_bytes().as_ref(),
            mint_count.to_le_bytes().as_ref(),
            &[bump],
        ]],
    )?;

    Ok(())
}

pub fn init_collection_mint_account<'info>(
    payer: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    rent_account: AccountInfo<'info>,
    extensions: &[ExtensionType],
    token_metadata: TokenMetadata,
    collection_id: u64,
) -> Result<()> {
    // Calculate extension sizes
    // let base_size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions).unwrap();
    let base_size =
        ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(extensions)
            .unwrap();
    let extension_extra_space = token_metadata.tlv_size_of().unwrap();

    let rent = &Rent::from_account_info(&rent_account)?;
    let lamports = rent.minimum_balance(base_size + extension_extra_space);
    let ix = solana_program::system_instruction::create_account(
        &payer.key(),
        &mint.key(),
        lamports,
        (base_size).try_into().unwrap(),
        &spl_token_2022::id(),
    );

    let account_infos: Vec<AccountInfo> = vec![payer, mint.clone()];
    let (expected_mint_account, bump) = Pubkey::find_program_address(
        &[SEED_COLLECTION_MINT, collection_id.to_le_bytes().as_ref()],
        &ID,
    );
    require_keys_eq!(expected_mint_account, mint.key());

    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&[
            SEED_COLLECTION_MINT,
            collection_id.to_le_bytes().as_ref(),
            &[bump],
        ]],
    )?;

    Ok(())
}

pub fn add_closing_authority(
    mint_account: &AccountInfo,
    program: Pubkey,
    program_delegate: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_mint_close_authority(
        &program,
        &mint_account.key(),
        Some(&program_delegate),
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}

pub fn add_transfer_hook(
    mint_account: &AccountInfo,
    program: Pubkey,
    authority: Pubkey,
    transfer_hook_program: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::extension::transfer_hook::instruction::initialize(
        &program,
        &mint_account.key(),
        Some(authority),
        Some(transfer_hook_program),
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}

pub fn add_permanent_delegate(
    mint_account: &AccountInfo,
    program: Pubkey,
    program_delegate: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_permanent_delegate(
        &program,
        &mint_account.key(),
        &program_delegate,
    )?;

    let account_infos: Vec<AccountInfo> = vec![mint_account.to_account_info()];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}
