use crate::*;
use solana_program::program_pack::Pack;

pub fn get_token_account_owner(token_account: &AccountInfo) -> Result<Pubkey> {
    let state =
        spl_token_2022::state::Account::unpack_from_slice(&token_account.try_borrow_data()?)?;
    Ok(state.owner)
}

pub fn update_token_metadata<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        program_id,
        &metadata.key(),
        &update_authority.key(),
        field,
        value,
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
    ];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}

pub fn update_token_metadata_signed<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        program_id,
        &metadata.key(),
        &update_authority.key(),
        field,
        value,
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
    ];

    // TODO not ideal
    // let (_, bump) = Pubkey::find_program_address(&[SEED_PROGRAM_DELEGATE], &ID);

    // let seeds = &[SEED_PROGRAM_DELEGATE, &[bump]];
    solana_program::program::invoke_signed(&ix, &account_infos[..], signer_seeds)?;

    Ok(())
}

pub fn burn_token<'info>(
    mint_account: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    program: Pubkey,
    authority: &AccountInfo<'info>,
    seeds: Option<&[&[u8]; 2]>,
) -> Result<()> {
    let ix = spl_token_2022::instruction::burn(
        &program,
        &token_account.key(),
        &mint_account.key(),
        &authority.key(),
        &[],
        1,
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        mint_account.to_account_info(),
        authority.to_account_info(),
    ];

    match seeds {
        Some(s) => solana_program::program::invoke_signed(&ix, &account_infos[..], &[&s[..]])?,
        None => solana_program::program::invoke(&ix, &account_infos[..])?,
    }

    Ok(())
}

pub fn close_mint<'info>(
    program: Pubkey,
    token_account: &AccountInfo<'info>,
    destination_account: &AccountInfo<'info>,
    owner: &AccountInfo<'info>,
    seeds: Option<&[&[u8]; 2]>,
) -> Result<()> {
    let ix = spl_token_2022::instruction::close_account(
        &program,
        &token_account.key(),
        &destination_account.key(),
        &owner.key(),
        &[],
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        destination_account.to_account_info(),
        owner.to_account_info(),
    ];

    match seeds {
        Some(s) => solana_program::program::invoke_signed(&ix, &account_infos[..], &[&s[..]])?,
        None => solana_program::program::invoke(&ix, &account_infos[..])?,
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn transfer_token_with_pda<'info>(
    amount: u64,
    decimals: u8,
    token_program: &AccountInfo<'info>,
    source_pubkey: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    destination_pubkey: &AccountInfo<'info>,
    authority_pubkey: &AccountInfo<'info>,
    signer_pubkeys: &[&Pubkey],
    seeds: &[&[u8]; 2],
) -> Result<()> {
    let ix = spl_token_2022::instruction::transfer_checked(
        token_program.key,
        source_pubkey.key,
        mint.key,
        destination_pubkey.key,
        authority_pubkey.key,
        signer_pubkeys,
        amount,
        decimals,
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        source_pubkey.to_account_info(),
        mint.to_account_info(),
        destination_pubkey.to_account_info(),
        authority_pubkey.to_account_info(),
    ];

    // TODO not ideal
    // let (_, bump) = Pubkey::find_program_address(&[SEED_PROGRAM_DELEGATE], &ID);
    // let program_delegate_seeds = &[SEED_PROGRAM_DELEGATE, &[bump]];

    solana_program::program::invoke_signed(&ix, &account_infos[..], &[seeds])?;

    Ok(())
}

pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    if extra_lamports > 0 {
        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                payer.key,
                account.key,
                extra_lamports,
            ),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
