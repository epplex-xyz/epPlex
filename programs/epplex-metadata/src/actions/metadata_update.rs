// Inspired by this
// https://solana.stackexchange.com/questions/9106/questions-on-token2022-metadata/9107#9107

use crate::*;

use anchor_lang::{
    system_program::{transfer, Transfer},
};

use spl_token_metadata_interface::{
    error::TokenMetadataError,
};
use spl_type_length_value::state::{
    realloc_and_pack_first_variable_len, TlvState, TlvStateBorrowed,
};

#[derive(Accounts)]
pub struct MetadataUpdate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: check by address only, no anchor type to check against
    #[account(
        seeds = [b"metadata", mint.key().as_ref()],
        bump
    )]
    pub metadata: UncheckedAccount<'info>,

    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // TODO removed Option here
    /// CHECK: Optional update authority, unchecked because it can either be SystemAccount or a PDA owned by another program
    pub update_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


impl MetadataUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &UpdateFieldData) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: UpdateFieldData) -> Result<()> {
        // Get current TokenMetadata.
        let mut token_metadata = {
            let buffer = ctx.accounts.metadata.try_borrow_data()?;
            let state = TlvStateBorrowed::unpack(&buffer)?;
            state.get_first_variable_len_value::<TM>()?
        } ;

        // Check update authority.
        let update_authority = Option::<Pubkey>::from(token_metadata.update_authority)
            .ok_or_else(|| ProgramError::Custom(TokenMetadataError::ImmutableMetadata as u32))?;
        msg!("Update authority: {:?}", update_authority);
        if update_authority != *ctx.accounts.update_authority.key {
            return Err(
                ProgramError::Custom(TokenMetadataError::IncorrectUpdateAuthority as u32).into(),
            );
        }

        // Perform the update on the TokenMetadata.
        let field = params.field.to_field();
        token_metadata.update(field, params.value);
        msg!("TokenMetadata: {:?}", token_metadata);

        // Calculate the required size and lamports for the updated metadata.
        let new_size = TM::tlv_size_of(&token_metadata)?;
        let required_lamports = Rent::get()?.minimum_balance(new_size as usize);

        // Get current state of the metadata account.
        let metadata_account_info = ctx.accounts.metadata.to_account_info();
        let current_lamports = metadata_account_info.lamports();

        // Transfer lamports if required.
        if required_lamports != current_lamports {
            let lamport_difference =
                (required_lamports as isize - current_lamports as isize).abs() as u64;
            if required_lamports > current_lamports {
                // Transfer additional lamports to metadata account.
                msg!(
                    "Transferring {} lamports to metadata account",
                    lamport_difference
                );
                transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.payer.to_account_info(),
                            to: metadata_account_info.clone(),
                        },
                    ),
                    lamport_difference,
                )?;
            } else {
                // Transfer excess lamports back to payer.
                msg!("Transferring {} lamports back to payer", lamport_difference);
                // Modify lamports directly because metadata account is owned by this program (and not System Program)
                ctx.accounts.metadata.sub_lamports(lamport_difference)?;
                ctx.accounts.payer.add_lamports(lamport_difference)?;
            }
        }

        // Reallocate and update the metadata account data.
        realloc_and_pack_first_variable_len(
            &ctx.accounts.metadata.to_account_info(),
            &token_metadata,
        )?;
        Ok(())
    }

}