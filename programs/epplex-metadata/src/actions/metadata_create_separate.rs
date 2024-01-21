// Inspired by this
// https://solana.stackexchange.com/questions/9106/questions-on-token2022-metadata/9107#9107

use crate::*;

use anchor_lang::{
    system_program::{create_account, CreateAccount,},
};
use solana_program::borsh0_10::get_instance_packed_len;
use spl_pod::optional_keys::OptionalNonZeroPubkey;


use spl_type_length_value::state::TlvStateMut;

#[derive(Accounts)]
#[instruction(params: MetadataCreateSeparateParams)]
pub struct MetadataCreateSeparate<'info> {
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

    /// CHECK: Optional update authority, unchecked because it can either be SystemAccount or a PDA owned by another program
    pub update_authority: Option<UncheckedAccount<'info>>,


    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MetadataCreateSeparateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl MetadataCreateSeparate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &MetadataCreateSeparateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: MetadataCreateSeparateParams) -> Result<()> {
        // Check if update authority is provided
        let update_authority_key = ctx
            .accounts
            .update_authority
            .as_ref()
            .map(|account| *account.key);

        let update_authority = OptionalNonZeroPubkey::try_from(update_authority_key)
            .map_err(|_| ProgramError::InvalidArgument)?;

        // Construct token metadata
        let token_metadata = TM {
            name: params.name,
            symbol: params.symbol,
            uri: params.uri,
            update_authority,
            mint: ctx.accounts.mint.key(),
            ..Default::default()
        };
        msg!("TokenMetadata: {:?}", token_metadata);

        // Calculate size and lamports for the metadata account
        let size = TM::tlv_size_of(&token_metadata)?;
        let lamports = Rent::get()?.minimum_balance(size as usize);

        // Create metadata account
        let mint = ctx.accounts.mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[b"metadata", mint.as_ref(), &[ctx.bumps.metadata]]];
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.metadata.to_account_info(),
                },
            )
                .with_signer(signer_seeds),
            lamports,
            size as u64,
            &ID,
        )?;

        // Initialize metadata account data
        let instance_size = get_instance_packed_len(&token_metadata)?;
        let mut buffer = ctx.accounts.metadata.try_borrow_mut_data()?;
        let mut state = TlvStateMut::unpack(&mut buffer)?;
        state.alloc::<TM>(instance_size, false)?;
        state.pack_first_variable_len_value(&token_metadata)?;

        Ok(())
    }

}