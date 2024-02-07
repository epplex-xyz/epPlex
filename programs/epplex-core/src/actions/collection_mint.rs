use crate::*;
use anchor_spl::token_interface::MintTo;
use spl_token_metadata_interface::state::TokenMetadata;
use epplex_shared::{Token2022, update_token_metadata};
use crate::mint::{COLLECTION_ID_FIELD, TokenCollectionCreateParams};

#[derive(Accounts)]
#[instruction(params: TokenCollectionCreateParams)]
pub struct CollectionMint<'info> {
    // TODO need to add checks
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // TODO need to add checks
    #[account(mut)]
    /// CHECK
    pub token_account: UncheckedAccount<'info>,

    // TODO Gate this endpoint to burger??? Prolly no need since we have our own PDA
    // #[account()]
    // /// CHECK in CPI
    // pub token_metadata: UncheckedAccount<'info>,

    #[account()]
    /// CHECK
    pub permanent_delegate: UncheckedAccount<'info>, // No need to sign, simply assigning

    #[account()]
    pub update_authority: Signer<'info>,

    //
    #[account(mut)]
    pub payer: Signer<'info>, // Payer for all the stuff

    #[account(mut, has_one = authority,
    seeds = [SEED_COLLECTION_CONFIG, &params.collection_id.to_le_bytes()],
    bump
    )]
    pub collection_config: Account<'info, CollectionConfig>,

    /// CHECK This is the admin account assigned when the collection is created.
    #[account(signer)]
    pub authority: UncheckedAccount<'info>,

    // #[account()]
    // /// CHECK
    // pub transfer_hook_program: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
}

impl CollectionMint<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCollectionCreateParams) -> Result<()> {
        Ok(())
    }

    // This function should be a general purpose minter
    pub fn actuate(ctx: Context<Self>, params: TokenCollectionCreateParams) -> Result<()> {
        let update_authority = spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(
            Some(ctx.accounts.update_authority.key())
        ).expect("Bad update auth");

        // Convert from Vec<[String;2]> to Vec<(String, String)>
        let mut converted_metadata: Vec<(String, String)> = params.additional_metadata
            .iter()
            .map(|array| (array[0].clone(), array[1].clone()))
            .collect();

        // Increment the mint count to create a new mint ID
        ctx.accounts.collection_config.mint_count += 1;
        converted_metadata.push((COLLECTION_ID_FIELD.to_string(), params.collection_id.to_string()));
        converted_metadata.push((MINT_COUNT_FIELD.to_string(), ctx.accounts.collection_config.mint_count.to_string()));


        let tm = TokenMetadata {
            update_authority,
            mint: ctx.accounts.mint.key(),
            name: params.name.clone(),
            symbol: params.symbol.clone(),
            uri: params.uri.clone(),
            additional_metadata: converted_metadata.clone()
        };

        // Create the ephemeral token
        init_mint_account(
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.rent.to_account_info().clone(),
            &[
                ExtensionType::MintCloseAuthority,
                ExtensionType::PermanentDelegate,
                ExtensionType::MetadataPointer,
                // ExtensionType::TransferHook
            ],
            tm
        )?;

        // Add ClosingAuth Extension
        add_closing_authority(
            &ctx.accounts.mint,
            ctx.accounts.token22_program.key(),
            ctx.accounts.permanent_delegate.key(),
        )?;

        // Add PermanentDelegate Extension
        add_permanent_delegate(
            &ctx.accounts.mint,
            ctx.accounts.token22_program.key(),
            ctx.accounts.permanent_delegate.key(),
        )?;

        // Add TransferHook Extension
        // add_transfer_hook(
        //     &ctx.accounts.mint,
        //     ctx.accounts.token22_program.key(),
        //     ctx.accounts.permanent_delegate.key(),
        //     ctx.accounts.transfer_hook_program.key(),
        // )?;

        // Add MetadataPointer Extension
        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.permanent_delegate.key(),
            ctx.accounts.mint.key(),
        )?;

        // In LibrePlex, the mint auth and freeze auth are the deployment pda
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

        // Initialize token metadata
        initialize_token_metadata(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO update auth
            &ctx.accounts.update_authority.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            // TODO: mint auth
            &ctx.accounts.payer,
            params.name.clone(),
            params.symbol.clone(),
            params.uri.clone(),
        )?;


        // Might need ot put into separate instruction
        // Add all the metadata
        for (field, value) in converted_metadata.into_iter() {
            update_token_metadata(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(), // Metadata on mint account
                &ctx.accounts.update_authority.to_account_info(),
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
                    associated_token: ctx.accounts.token_account.to_account_info(),
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
                    to: ctx.accounts.token_account.to_account_info().clone(),
                    authority: ctx.accounts.payer.to_account_info(),
                }
            ),
            1
        )?;

        // TODO in LibrePlex case the authority is a PDA
        // TODO prolly need to do the same

        // Remove freeze auth
        anchor_spl::token_interface::set_authority(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::token_interface::SetAuthority {
                    current_authority:  ctx.accounts.payer.to_account_info().clone(),
                    account_or_mint: ctx.accounts.mint.to_account_info().clone(),
                },
                // &[deployment_seeds]
            ),
            anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::FreezeAccount,
            None, // Set authority to be None
        )?;

        // Removing mint authority
        anchor_spl::token_interface::set_authority(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::token_interface::SetAuthority {
                    current_authority: ctx.accounts.payer.to_account_info().clone(),
                    account_or_mint: ctx.accounts.mint.to_account_info().clone(),
                },
                // &[deployment_seeds]
            ),
            anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::MintTokens,
            None, // Set mint authority to be None
        )?;

        Ok(())
    }

}