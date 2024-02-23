use anchor_spl::associated_token::AssociatedToken;
use epplex_core::GlobalCollectionConfig;
use epplex_core::program::EpplexCore;
use crate::*;

#[derive(Accounts)]
#[instruction(params: WhitelistMintParams)]
pub struct WhitelistMint<'info> {
    #[account(mut)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_account: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        payer = payer,
        space = BurgerMetadata::LEN,
        bump,
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub epplex_core: Program<'info, EpplexCore>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhitelistMintParams {
    pub expiry_date: String,
    pub name: String,
    pub symbol: String,
    pub uri: String
}

impl WhitelistMint<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &WhitelistMintParams) -> Result<()> {
        let expiry_date =  params.expiry_date.parse::<i64>().unwrap();
        let now = Clock::get().unwrap().unix_timestamp;
        if !(now < expiry_date) {
            return err!(BurgerError::DateMustBeInTheFuture);
        }

        // Maybe need to check for link in URI

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: WhitelistMintParams) -> Result<()> {
        // Create the burger metadata
        let token_metadata = &mut ctx.accounts.token_metadata;
        **token_metadata = BurgerMetadata::new(
            ctx.bumps.token_metadata,
        );

        let additional_metadata = generate_metadata(params.expiry_date);


        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.permanent_delegate.bump]];
        // CPI into token_mint
        epplex_core::cpi::token_mint(
            CpiContext::new_with_signer(
                ctx.accounts.epplex_core.to_account_info(),
                epplex_core::cpi::accounts::TokenMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    token_account: ctx.accounts.token_account.to_account_info(),
                    permanent_delegate: ctx.accounts.permanent_delegate.to_account_info(),
                    update_authority: ctx.accounts.permanent_delegate.to_account_info(), // update_auth = perm_delegate
                    payer: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token22_program: ctx.accounts.token22_program.to_account_info(),
                    associated_token: ctx.accounts.associated_token.to_account_info(),
                    global_collection_config: ctx.accounts.global_collection_config.to_account_info(),
                },
                &[&seeds[..]]
            ),
            epplex_core::mint::TokenCreateParams {
                name: params.name,
                symbol: params.symbol,
                uri: params.uri,
                additional_metadata: additional_metadata
            },
        )
    }
}
