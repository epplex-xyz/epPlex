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
        //
        // // TODO This one is probably missing the permanent delegate extension
        // // TODO Probably need to clearly define what this does
        // // Create the ephemeral token
        // token_create_basic(
        //     ctx.accounts.mint.to_account_info().clone(),
        //     ctx.accounts.program_delegate.to_account_info().clone(),
        //     ctx.accounts.payer.to_account_info().clone(),
        //     ctx.accounts.rent.to_account_info().clone(),
        //     ctx.accounts.token22_program.to_account_info().clone(),
        //     &[ExtensionType::MetadataPointer]
        // )?;
        //
        //
        // // Create token metadata account
        // create_metadata_account(
        //     ctx.accounts.metadata_program.to_account_info().clone(),
        //     ctx.accounts.payer.to_account_info().clone(),
        //     ctx.accounts.mint.to_account_info().clone(),
        //     ctx.accounts.token_metadata.to_account_info().clone(),
        //     ctx.accounts.system_program.to_account_info().clone(),
        //     params
        // )?;
        //
        // // Point to created the metadata account
        // add_metadata_pointer(
        //     ctx.accounts.token22_program.key(),
        //     &ctx.accounts.mint.to_account_info(),
        //     ctx.accounts.program_delegate.key(),
        //     ctx.accounts.token_metadata.key()
        // )?;
        //
        // // Initialize the actual mint data
        // initialize_mint(
        //     &ctx.accounts.mint.to_account_info(),
        //     &ctx.accounts.rent.to_account_info(),
        //     &ctx.accounts.token22_program.key(),
        //     // TODO incorrect
        //     &ctx.accounts.payer.key(),
        //     // TODO incorrect
        //     &ctx.accounts.payer.key(),
        // )?;
        //
        // // Create ATA
        // anchor_spl::associated_token::create(
        //     CpiContext::new(
        //         ctx.accounts.token22_program.to_account_info(),
        //         anchor_spl::associated_token::Create {
        //             payer: ctx.accounts.payer.to_account_info(), // payer
        //             associated_token: ctx.accounts.ata.to_account_info(),
        //             authority: ctx.accounts.payer.to_account_info(), // owner
        //             mint: ctx.accounts.mint.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token22_program.to_account_info(),
        //         }
        //     ),
        // )?;
        //
        // // Mint to ATA
        // anchor_spl::token_interface::mint_to(
        //     CpiContext::new(
        //         ctx.accounts.token22_program.to_account_info(),
        //         MintTo {
        //             mint: ctx.accounts.mint.to_account_info().clone(),
        //             to: ctx.accounts.ata.to_account_info().clone(),
        //             authority: ctx.accounts.payer.to_account_info(),
        //         }
        //     ),
        //     1
        // )?;

        Ok(())
    }

}
