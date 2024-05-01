use solana_program::system_instruction;

use crate::*;

// Probably system program already has this callable, I just copied from there
pub fn transfer_sol<'info>(
    program_id: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let ix = system_instruction::transfer(&from.key(), &to.key(), amount);

    let account_infos: Vec<AccountInfo> = vec![
        from.to_account_info(),
        to.to_account_info(),
        program_id.to_account_info(),
    ];

    solana_program::program::invoke(&ix, &account_infos[..])?;

    Ok(())
}
