use crate::*;
use anchor_lang::solana_program;
use solana_program::program_pack::Pack;

pub fn get_token_account_owner(token_account: &AccountInfo) -> Result<Pubkey> {
    let state =
        spl_token_2022::state::Account::unpack_from_slice(&token_account.try_borrow_data()?)?;
    Ok(state.owner)
}

// prolly could use the shared function
pub fn update_token_metadata_signed<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        // &program_id,
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
    let (_, bump) = Pubkey::find_program_address(&[SEED_PROGRAM_DELEGATE], &ID);

    let seeds = &[SEED_PROGRAM_DELEGATE, &[bump]];
    solana_program::program::invoke_signed(&ix, &account_infos[..], &[&seeds[..]])?;

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
    let (_, bump) = Pubkey::find_program_address(&[SEED_PROGRAM_DELEGATE], &ID);
    let program_delegate_seeds = &[SEED_PROGRAM_DELEGATE, &[bump]];

    solana_program::program::invoke_signed(&ix, &account_infos[..], &[program_delegate_seeds])?;

    Ok(())
}
