use anchor_spl::associated_token::AssociatedToken;
use epplex_core::program::EpplexCore;
use crate::*;

#[derive(Accounts)]
#[instruction(params: WhitelistMintParams)]
pub struct WhitelistMint<'info> {
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub ata: UncheckedAccount<'info>,

    // #[account(mut)]
    // /// CHECK
    // pub token_metadata: UncheckedAccount<'info>,

    // TODO: is unchecked account correct?
    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    /// CHECK
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub epplex_core: Program<'info, EpplexCore>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhitelistMintParams {
    pub destroy_timestamp: String,
    pub name: String,
    pub symbol: String,
    pub uri: String
}

impl WhitelistMint<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &WhitelistMintParams) -> Result<()> {
        // TODO need to check for destroy timestamp
        //  need to do some validations
        // let now = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: WhitelistMintParams) -> Result<()> {
        let additional_metadata = vec![
            [EXPIRY_FIELD.to_string(), params.destroy_timestamp],
            [RENEWAL_FIELD.to_string(), "0".to_string()],
            [FOR_SALE_FIELD.to_string(), "0".to_string()],
            [PRICE_FIELD.to_string(), "9999".to_string()],
            [GAME_STATE.to_string(), "0".to_string()]
        ];

        // CPI into token_mint
        epplex_core::cpi::token_mint(
            CpiContext::new(
                ctx.accounts.epplex_core.to_account_info(),
                epplex_core::cpi::accounts::TokenMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    ata: ctx.accounts.ata.to_account_info(),
                    permanent_delegate: ctx.accounts.permanent_delegate.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token22_program: ctx.accounts.token22_program.to_account_info(),
                    associated_token: ctx.accounts.associated_token.to_account_info()
                }
            ),
            epplex_core::TokenCreateParams {
                name: params.name,
                symbol: params.symbol,
                uri: params.uri,
                additional_metadata: additional_metadata
            }
        )
    }
}
