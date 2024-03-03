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
