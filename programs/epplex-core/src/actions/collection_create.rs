use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::MintTo;
use epplex_shared::update_token_metadata;

#[derive(Accounts)]
#[instruction(params: CollectionCreateParams)]
pub struct CollectionCreate<'info> {
    #[account(
        init,
        seeds = [
            SEED_COLLECTION_CONFIG,
            global_collection_config.collection_counter.to_le_bytes().as_ref()
        ],
        bump,
        payer = payer,
        space = CollectionConfig::LEN,
    )]
    /// CHECK
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(
        mut,
        seeds = [
            SEED_GLOBAL_COLLECTION_CONFIG
        ],
        bump = global_collection_config.bump
    )]
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(
        mut,
        constraint = epplex_shared::ADMINS.contains(
            &payer.key()
        )
    )]
    pub payer: Signer<'info>,

    /// CHECK this account is created in the instruction body, so no need to check data layout
    #[account(
        mut,
        seeds = [
            SEED_COLLECTION_MINT,
            global_collection_config.collection_counter.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub mint: UncheckedAccount<'info>,

    /// CHECK this account is created in the instruction body, so no need to check data layout
    #[account(
        mut,
        seeds = [
            payer.key().as_ref(),
            token22_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = associated_token_program.key(),
        bump
    )]
    pub token_account: UncheckedAccount<'info>,

    pub update_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCreateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub authority: Pubkey,
    pub renewal_price: u64,
    pub mint_price: u64,
    pub standard_duration: u32,
    pub grace_period: i64,
    pub treasury: Pubkey,
    pub collection_size: u32,
}

impl CollectionCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &CollectionCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: CollectionCreateParams) -> Result<()> {
        let config = &mut ctx.accounts.collection_config;
        **config = CollectionConfig::new(ctx.bumps.collection_config, params.clone());

        let global_config = &mut ctx.accounts.global_collection_config;
        let additional_metadata: Vec<(String, String)> = vec![(
            COLLECTION_ID_FIELD.to_string(),
            global_config.collection_counter.to_string(),
        )];

        let update_authority = anchor_spl::token_interface::spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(Some(
            ctx.accounts.update_authority.key(),
        ))
        .expect("Bad update auth");

        let tm = TokenMetadata {
            update_authority,
            mint: ctx.accounts.mint.key(),
            name: params.name.clone(),
            symbol: params.symbol.clone(),
            uri: params.uri.clone(),
            additional_metadata: additional_metadata.clone(),
        };
        init_collection_mint_account(
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            &[ExtensionType::MetadataPointer],
            tm,
            global_config.collection_counter,
        )?;

        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.update_authority.key(),
            ctx.accounts.mint.key(),
        )?;

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
            // Update auth
            &ctx.accounts.update_authority.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            // mint auth
            &ctx.accounts.update_authority.to_account_info(),
            params.name.clone(),
            params.symbol.clone(),
            params.uri.clone(),
        )?;

        for (field, value) in additional_metadata.into_iter() {
            update_token_metadata(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(), // Metadata on mint account
                &ctx.accounts.update_authority.to_account_info(),
                anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Key(field),
                value,
            )?;
        }

        // Create ATA
        anchor_spl::associated_token::create(CpiContext::new(
            ctx.accounts.token22_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.payer.to_account_info(), // payer
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.update_authority.to_account_info(), // owner
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
        ))?;

        // Mint to ATA
        anchor_spl::token_interface::mint_to(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info().clone(),
                    to: ctx.accounts.token_account.to_account_info().clone(),
                    authority: ctx.accounts.update_authority.to_account_info(),
                },
            ),
            1,
        )?;

        // Remove freeze auth
        anchor_spl::token_interface::set_authority(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::token_interface::SetAuthority {
                    current_authority: ctx.accounts.update_authority.to_account_info().clone(),
                    account_or_mint: ctx.accounts.mint.to_account_info().clone(),
                },
            ),
            anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::FreezeAccount,
            None, // Set authority to be None
        )?;

        // Removing mint authority
        anchor_spl::token_interface::set_authority(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                anchor_spl::token_interface::SetAuthority {
                    current_authority: ctx.accounts.update_authority.to_account_info().clone(),
                    account_or_mint: ctx.accounts.mint.to_account_info().clone(),
                },
            ),
            anchor_spl::token_2022::spl_token_2022::instruction::AuthorityType::MintTokens,
            None, // Set mint authority to be None
        )?;

        global_config.collection_counter += 1;
        Ok(())
    }
}
