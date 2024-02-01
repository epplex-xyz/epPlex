// use anchor_spl::token_interface::MintTo;
use epplex_metadata::program::EpplexMetadata;
use epplex_shared::Token2022;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct CollectionMint<'info> {
    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub ata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_metadata: UncheckedAccount<'info>,

    #[account()]
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(mut)]
    ///CHECK: Checked in validate
    pub treasury: AccountInfo<'info>,

    // #[account(
    //     seeds = [SEED_PROGRAM_DELEGATE],
    //     bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, EpplexMetadata>
}

impl CollectionMint<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        // TODO use as constraints -- why aren't these checks inside the colleciton mint funciton?
        if ctx.accounts.mint_authority.key() != ctx.accounts.collection_config.authority {
            return err!(MintError::UnauthorizedMintAuthority)
        };

        if ctx.accounts.treasury.key() != ctx.accounts.collection_config.treasury {
            return err!(MintError::InvalidTreasuryAccount)
        };

        Ok(())
    }

    // TODO all of this logic is probably redundant, should just call token_mint directly from epplex mint
    // we don't like duplicate code, why is this duplicated?
    // basically the same function as token_mint, should just actuate token_mint
    pub fn actuate(_ctx: Context<Self>, _params: TokenCreateParams) -> Result<()> {
        // Transfer mint price to treasury
        // transfer_sol(
        //     &ctx.accounts.system_program,
        //     &ctx.accounts.payer,
        //     &ctx.accounts.treasury,
        //     ctx.accounts.collection_config.mint_price
        // )?;

        // TODO call tokenMint here

        Ok(())
    }

}
