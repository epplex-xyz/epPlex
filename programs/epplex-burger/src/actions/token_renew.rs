use std::ops::{Add, Sub};
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {
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

    #[account(
        mut,
        associated_token::mint = mint_payment,
        associated_token::authority = VAULT_PUBKEY,
    )]
    pub proceeds_token_account: Account<'info, TokenAccount>, // Deposit in here

    #[account(
        mut,
        associated_token::mint = proceeds_token_account.mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>, // Deduct from here

    // Why do we have two signers here
    #[account(mut)]
    pub payer: Signer<'info>,

    // TODO: test in case not authority
    #[account(mut)]
    pub update_authority: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenRenewParams {}

impl TokenRenew<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenRenewParams,
    ) -> Result<()> {
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
            ctx.accounts.mint_payment.decimals
        )?;

        // TODO Check update auth
        // fetched_metadata.update_authority

        let expiry_date_string = fetch_metadata_field(EXPIRY_FIELD, &ctx.accounts.mint.to_account_info())?;
        let expiry_date =  expiry_date_string.parse::<i64>().unwrap();
        msg!("Destroy timestamp: {}", expiry_date);

        // Cannot exceed expiry
        // Disall
        let now = Clock::get().unwrap().unix_timestamp;
        if now > expiry_date {
            return err!(BurgerError::ExpiryDateHasBeenExceeded);
        }

        // Needs to be within 1 day of expiry date
        let threshold = expiry_date.sub(ONE_DAY);
        if !(threshold < now) {
            return err!(BurgerError::RenewThreshold);
        }

        let new_expiry_date = expiry_date.add(ONE_DAY).to_string();
        msg!("new timestamp: {}", new_expiry_date);
        // otherwise needs to do invoke signed, if authority is not the payer.
        epplex_shared::update_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO rethink this, who is allowed - prolly the update auth upon mint creation, needs to test with a PDA
            &ctx.accounts.update_authority.to_account_info(),
            spl_token_metadata_interface::state::Field::Key(EXPIRY_FIELD.to_string()),
            new_expiry_date
        )?;

        Ok(())
    }
}
