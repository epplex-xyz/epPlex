use std::ops::Add;
use crate::*;
use spl_token_2022::extension::BaseStateWithExtensions;

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {

    // TODO: add normal token program constraint
    #[account(
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
        constraint = SUPPORTED_TOKENS.contains(
            &mint_payment.key()
        ) @ BurgerError::TokenNotSupported
    )]
    pub mint_payment: Account<'info, Mint>,

    // TOOD add other checks here
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    // TODO need to modify authority
    #[account(
        mut,
        associated_token::mint = mint_payment,
        associated_token::authority = buyer,
    )]
    pub proceeds_token_account: Account<'info, TokenAccount>, // Deposit in here

    #[account(
        mut,
        associated_token::mint = proceeds_token_account.mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, // Deduct from here

    #[account(
        token::mint = mint,
        token::authority = authority,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(mut)]
    pub buyer: Signer<'info>, // Auth of buyer_token_account, mut for being payer

    // #[account(
    //     mut,
    //     seeds = [SEED_PROGRAM_DELEGATE],
    //     bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,
    #[account()]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    // TODO: test in case not authority
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenRenewParams {
    renew_terms: u16,
}

impl TokenRenew<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenRenewParams,
    ) -> Result<()> {
        // TODO prolly need to match on some update auth

        // from metaplex Metadata.rs

        // // Only the Update Authority can update this section.
        // match &args {
        //     UpdateArgs::V1 {
        //         new_update_authority,
        //         uses,
        //         collection_details,
        //         ..
        //     }
        //     | UpdateArgs::AsUpdateAuthorityV2 {
        //         new_update_authority,
        //         uses,
        //         collection_details,
        //         ..
        //     } => {
        //         if let Some(authority) = new_update_authority {
        //             self.update_authority = *authority;
        //         }
        //
        //         if uses.is_some() {
        //             let uses_option = uses.clone().to_option();
        //             // If already None leave it as None.
        //             assert_valid_use(&uses_option, &self.uses)?;
        //             self.uses = uses_option;
        //         }
        //
        //         if let CollectionDetailsToggle::Set(collection_details) = collection_details {
        //             // only unsized collections can have the size set, and only once.
        //             if self.collection_details.is_some() {
        //                 return Err(MetadataError::SizedCollection.into());
        //             }
        //
        //             self.collection_details = Some(collection_details.clone());
        //         }
        //     }
        //     _ => (),
        // }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenRenewParams) -> Result<()> {
        // Currently just SOL
        // Take payment 1 BONK
        let amount = u64::pow(10, ctx.accounts.mint_payment.decimals as u32);
        token_2022::transfer_checked(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.buyer_token_account.to_account_info(),
                    to: ctx.accounts.proceeds_token_account.to_account_info(),
                    mint: ctx.accounts.mint_payment.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.mint_payment.decimals
        )?;

        // TODO need to turn this into a helper function
        let buffer = ctx.accounts.mint.to_account_info();
        let expiry_date;

        // Fetch the expiry date
        {
            // Scoping because borrowing later
            let mint_data = buffer.try_borrow_data()?;
            let state = spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
            let metadata_bytes = state.get_extension_bytes::<TokenMetadata>().unwrap();
            let fetched_metadata = TokenMetadata::try_from_slice(metadata_bytes).unwrap();

            // TODO Check update auth
            // fetched_metadata.update_authority

            // TODO This is hardcoded, better to index on string
            let temp = fetched_metadata.additional_metadata[0].1.clone();
            expiry_date = temp.parse::<i64>().unwrap();
            msg!("Destroy timestamp: {}", temp);
        }

        let now = Clock::get().unwrap().unix_timestamp;
        if now > expiry_date {
            msg!("ExpiryDate already exceeded");
        }

        let new_expiry_date = expiry_date.add(ONE_WEEK).to_string();
        msg!("new timestamp: {}", new_expiry_date);
        update_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO rethink this
            &ctx.accounts.authority.to_account_info(), // who is allowed to make changes here? Changes have to go through program?
            spl_token_metadata_interface::state::Field::Key(EXPIRY_FIELD.to_string()),
            new_expiry_date
        )?;

        Ok(())
    }
}
