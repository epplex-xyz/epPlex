use crate::*;

use epplex_core::{TokenCreateParams};
use epplex_metadata::program::EpplexMetadata;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: CollectionMintFromParams)]
pub struct CollectionMintFrom<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,

    #[account(
        mut,
        seeds = [
            GUARD_SEED,
            collection_config.key().as_ref()
        ],
        bump = mint_guard.bump
    )]
    pub mint_guard: Account<'info, MintGuard>,

    pub epplex_program: Program<'info, EpplexCore>,

    // TODO: need to place constraint on collection_name
    #[account(
        seeds = [
            SEED_COLLECTION_CONFIG,
            &mint_guard.collection_counter.to_le_bytes(),
        ],
        seeds::program = epplex_core::ID.key(),
        bump = collection_config.bump
    )]
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(mut, signer)]
    /// CHECK
    pub token_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub ata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, EpplexMetadata>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionMintFromParams {

}

impl CollectionMintFrom<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &CollectionMintFromParams) -> Result<()> {
        let mint_guard = &ctx.accounts.mint_guard;
        let collection_config = &ctx.accounts.collection_config;

        if collection_config.collection_size <= mint_guard.items_minted {
            return err!(MintError::CollectionMintedOut);
        };

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: CollectionMintFromParams) -> Result<()> {
        let collection_config = &mut ctx.accounts.collection_config;
        let mint_guard = &mut ctx.accounts.mint_guard;

        // Create cpi
        let cpi_accounts = epplex_core::cpi::accounts::CollectionMint {
            mint: ctx.accounts.token_mint.to_account_info().clone(),
            ata: ctx.accounts.ata.to_account_info().clone(),
            token_metadata: ctx.accounts.token_metadata.to_account_info().clone(),
            program_delegate: ctx.accounts.program_delegate.to_account_info().clone(),
            payer: ctx.accounts.minter.to_account_info().clone(),
            rent: ctx.accounts.rent.to_account_info().clone(),
            token22_program: ctx.accounts.token22_program.to_account_info().clone(),
            system_program: ctx.accounts.system_program.to_account_info().clone(),
            associated_token: ctx.accounts.associated_token.to_account_info().clone(),
            collection_config: collection_config.to_account_info().clone(),
            mint_authority: mint_guard.to_account_info().clone(),
            treasury: mint_guard.to_account_info().clone(),
            metadata_program: ctx.accounts.metadata_program.to_account_info().clone()
        };

        // Create token creation params
        let mut token_name = collection_config.collection_name.clone();
        token_name.push_str(&mint_guard.items_minted.to_string()); // TODO does this add a space?
        let collection_symbol = collection_config.collection_symbol.clone();

        // TODO don't hardcode
        let params = TokenCreateParams {
            destroy_timestamp_offset: 604800,
            name: token_name,
            symbol: collection_symbol,
            uri: "".to_string()
        };

        let collection_config_key = collection_config.key();

        let seeds = &[GUARD_SEED, collection_config_key.as_ref(), &[mint_guard.bump]];
        epplex_core::cpi::collection_mint(
            CpiContext::new_with_signer(
                ctx.accounts.epplex_program.to_account_info(),
                cpi_accounts,
                &[seeds]
            ),
            params
        )?;

        mint_guard.items_minted += 1;

        Ok(())
    }

}
