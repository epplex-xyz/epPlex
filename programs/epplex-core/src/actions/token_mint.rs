use crate::*;
use epplex_metadata::program::EpplexMetadata;
use anchor_spl::token_interface::MintTo;
use spl_token_metadata_interface::state::TokenMetadata;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenMint<'info> {
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
    #[account()]
    /// CHECK
    pub permanent_delegate: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    // pub metadata_program: Program<'info, EpplexMetadata>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: Vec<(String, String)>,
}

impl TokenMint<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    // This function should be a general purpose minter
    pub fn actuate(ctx: Context<Self>, params: TokenCreateParams) -> Result<()> {
        // TODO set to permanent delegate for now
        let update_authority = spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(
            Some(ctx.accounts.permanent_delegate.key())
        ).expect("Bad update auth");

        let tm = TokenMetadata {
            update_authority,
            mint: ctx.accounts.mint.key(),
            name: params.name.clone(),
            symbol: params.symbol.clone(),
            uri: params.uri.clone(),
            additional_metadata: params.additional_metadata.clone()
        };

        // Create the ephemeral token
        init_mint_account(
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.rent.to_account_info().clone(),
            &[
                ExtensionType::PermanentDelegate,
                ExtensionType::MintCloseAuthority,
                ExtensionType::MetadataPointer
            ],
            tm
        )?;

        // TODO Need to create a separate PDA that simply has a bump - can do this later
        // Could also simply check for the permanent delegate address

        // Create metadata account
        // create_metadata_account(
        //     ctx.accounts.metadata_program.to_account_info().clone(),
        //     ctx.accounts.payer.to_account_info().clone(),
        //     ctx.accounts.mint.to_account_info().clone(),
        //     ctx.accounts.token_metadata.to_account_info().clone(),
        //     ctx.accounts.system_program.to_account_info().clone(),
        //     params
        // )?;

        // Add metadata pointer
        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.permanent_delegate.key(),
            ctx.accounts.mint.key(),
        )?;

        // Initialize the actual mint data
        initialize_mint(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.rent.to_account_info(),
            &ctx.accounts.token22_program.key(),
            // TODO incorrect mint auth
            &ctx.accounts.payer.key(),
            // TODO incorrect freeze auth
            &ctx.accounts.payer.key(),
        )?;
        // In LibrePlex, the mint auth and freeze auth are the deployment pda



        // Initialize token metadata
        add_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO update auth
            &ctx.accounts.permanent_delegate.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            // TODO: mint auth
            &ctx.accounts.payer,
            params.name.clone(),
            params.symbol.clone(),
            params.uri.clone(),
        )?;

        // Add all the metadata
        for (field, value) in params.additional_metadata.clone().into_iter() {
            update_token_metadata(
                &ctx.accounts.token22_program.key(),
                // Metadata on mint account
                &ctx.accounts.mint.to_account_info(),
                // TODO: mint auth
                &ctx.accounts.payer.to_account_info(),
                spl_token_metadata_interface::state::Field::Key(field),
                value,
            )?;
        }

        // Create ATA
        anchor_spl::associated_token::create(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.payer.to_account_info(), // payer
                    associated_token: ctx.accounts.ata.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(), // owner
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                }
            ),
        )?;

        // Mint to ATA
        anchor_spl::token_interface::mint_to(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info().clone(),
                    to: ctx.accounts.ata.to_account_info().clone(),
                    authority: ctx.accounts.payer.to_account_info(),
                }
            ),
            1
        )?;

        // TODO after minting should prolly burn the mint auth

        Ok(())
    }

}