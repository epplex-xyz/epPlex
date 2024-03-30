use crate::*;
use std::ops::{Add, Sub};

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {
    /// Technically anyone could pay to renew but why would they?
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
        constraint = SUPPORTED_TOKENS.contains(
            &mint_payment.key()
        ) @ BurgerError::TokenNotSupported
    )]
    pub mint_payment: Account<'info, Mint>,

    // This needs to always be ADMIN_PUBKEY account
    #[account(
        mut,
        associated_token::mint = mint_payment,
        associated_token::authority = epplex_shared::ADMIN_PUBKEY,
    )]
    pub proceeds_token_account: Account<'info, TokenAccount>, // Deposit in here

    #[account(
        mut,
        associated_token::mint = mint_payment,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>, // Deduct from here

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = update_authority.bump
    )]
    pub update_authority: Account<'info, ProgramDelegate>,

    pub token22_program: Program<'info, Token2022>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenRenewParams {}

impl TokenRenew<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenRenewParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenRenewParams) -> Result<()> {
        // Currently just SOL
        // TODO Take payment 1 BONK
        let amount = u64::pow(10, ctx.accounts.mint_payment.decimals as u32);
        token_2022::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.proceeds_token_account.to_account_info(),
                    mint: ctx.accounts.mint_payment.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.mint_payment.decimals,
        )?;

        let expiry_date_string =
            fetch_metadata_field(EXPIRY_FIELD, &ctx.accounts.mint.to_account_info())?;
        let expiry_date = expiry_date_string.parse::<i64>().unwrap();
        msg!("Destroy timestamp: {}", expiry_date);

        // Cannot exceed expiry - disallow renewal if time has surpassed
        let now = Clock::get().unwrap().unix_timestamp;
        if now > expiry_date {
            return err!(BurgerError::ExpiryDateHasBeenExceeded);
        }

        // Needs to be within 1 day of expiry date
        let threshold = expiry_date.sub(ONE_DAY);
        if threshold >= now {
            // if !(threshold < now) { -- same
            return err!(BurgerError::RenewThreshold);
        }

        let new_expiry_date = expiry_date.add(ONE_DAY).to_string();
        msg!("new timestamp: {}", new_expiry_date);

        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(),
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(EXPIRY_FIELD.to_string()),
            new_expiry_date,
        )?;

        Ok(())
    }
}
