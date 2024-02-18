use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: TokenBurnParams)]
pub struct TokenBurn<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        token::mint = mint.key(),
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        mut,
        close = payer,
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(
        mut,
        address = ADMIN_PUBKEY
    )]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBurnParams {}

impl TokenBurn<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        _params: &TokenBurnParams,
    ) -> Result<()> {
        let expiry_date_string = fetch_metadata_field(EXPIRY_FIELD, &ctx.accounts.mint.to_account_info())?;
        let expiry_date =  expiry_date_string.parse::<i64>().unwrap();

        // Cannot exceed expiry
        let now = Clock::get().unwrap().unix_timestamp;
        msg!("Destroy timestamp: {:?}, now {:?}", expiry_date, now);
        if now < expiry_date {
            return err!(BurgerError::NotYetExpired);
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenBurnParams) -> Result<()> {
        // Close the metadata account

        burn_token(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.token_account.to_account_info(),
            ctx.accounts.token22_program.key(),
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        close_mint(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // Currently rent collector is hardcoded to be the Program Delegaate
            &ctx.accounts.payer.to_account_info(),
            // Authority to close the mint
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        // Can only close the ATA if we are the owners
        let ata_owner = ctx.accounts.token_account.to_account_info().owner;
        if ata_owner == ctx.accounts.payer.owner {
            anchor_spl::token_interface::close_account(
                CpiContext::new(
                    ctx.accounts.token22_program.to_account_info(),
                    anchor_spl::token_interface::CloseAccount {
                        account: ctx.accounts.token_account.to_account_info().clone(),
                        destination: ctx.accounts.payer.to_account_info().clone(),
                        authority: ctx.accounts.payer.to_account_info().clone(),
                    },
                ),
            )?;
        }

        // Another one bites the dust
        ctx.accounts.game_config.bump_burn_amount()?;

        Ok(())
    }
}
