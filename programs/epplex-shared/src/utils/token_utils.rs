use crate::*;

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
