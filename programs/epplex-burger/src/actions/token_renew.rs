use std::ops::Add;
use crate::*;
use epplex_shared::Token2022;
use spl_token_2022::extension::BaseStateWithExtensions;

#[derive(Accounts)]
#[instruction(params: TokenRenewParams)]
pub struct TokenRenew<'info> {
    // TOOD add other checks here
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        token::mint = mint,
        token::authority = authority,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    // #[account(
    //     mut,
    //     seeds = [SEED_PROGRAM_DELEGATE],
    //     bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,
    #[account()]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    // TODO: test not authority
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

    pub fn actuate(ctx: Context<Self>, _params: &TokenRenewParams) -> Result<()> {
        // TODO
        // How to enforce that you can only renew once?
        // you would need to store the old destroy_timestamp somewhere to maintain a range.

        // TODO need to accept payments
        // param should simply
        // should compute the costs


        // Scoping because borrowing later
        // TODO need to turn this into a helper function
        let buffer = ctx.accounts.mint.to_account_info();
        let expiry_date;
        {
            let mint_data = buffer.try_borrow_data()?;
            let state = spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
            let metadata_bytes = state.get_extension_bytes::<TokenMetadata>().unwrap();
            let fetched_metadata = TokenMetadata::try_from_slice(metadata_bytes).unwrap();

            let temp = fetched_metadata.additional_metadata[0].1.clone();
            expiry_date = temp.parse::<i64>().unwrap();

            msg!("Destroy timestamp: {}", temp);
            let now = Clock::get().unwrap().unix_timestamp;
            if now > expiry_date {
                msg!("ExpiryDate already exceeded");
            }

            msg!("new timestamp: {}", expiry_date.add(ONE_WEEK));
        }

        update_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO rethink this
            &ctx.accounts.authority.to_account_info(), // who is allowed to make changes here? Changes have to go through program?
            spl_token_metadata_interface::state::Field::Key(EXPIRY_FIELD.to_string()),
            expiry_date.add(ONE_WEEK).to_string()
        )?;

        Ok(())
    }
}
