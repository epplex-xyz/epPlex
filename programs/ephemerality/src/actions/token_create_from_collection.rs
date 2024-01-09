use anchor_spl::{
    token_interface::MintTo
};
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateFromCollectionParams)]
pub struct TokenCreateFromCollection<'info> {
    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // TODO does this ensure consistent ATA seeds?
    // TODO: is it possible to use this?
    // TODO: ensure this is token2022 account
    // #[account(
    //     init,
    //     payer = payer,
    //     associated_token::authority = payer,
    //     associated_token::mint = mint,
    // )]
    // pub ata: Account<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK
    pub ata: UncheckedAccount<'info>,

    // TODO how to get exact space of this?
    #[account(
        init,
        seeds = [
            SEED_TOKEN_METADATA,
            ID.as_ref(),
            mint.key().as_ref()
        ],
        payer = payer,
        space = EphemeralMetadata::LEN,
        bump,
    )]
    pub token_metadata: Account<'info, EphemeralMetadata>,

    #[account()]
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub mint_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateFromCollectionParams {
    pub destroy_timestamp_offset: i64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl TokenCreateFromCollection<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &TokenCreateFromCollectionParams) -> Result<()> {

        if ctx.accounts.mint_authority.key() != ctx.accounts.collection_config.authority {
            return err!(MintError::UnauthorizedMintAuthority)
        };

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenCreateFromCollectionParams) -> Result<()> {
        // Initialise Mint Account
        let extension_sizes = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &[ExtensionType::PermanentDelegate, ExtensionType::MintCloseAuthority]
        ).unwrap();
        let rent = &Rent::from_account_info(&ctx.accounts.rent.to_account_info())?;
        // TODO need to have collectionConfig passed in

        // TODO: all NFTs should have same expiration date upon mint
        // maybe just save the now date and the destroytimeoffset

        // TODO: need to calculate this properly
        let space = extension_sizes + (64 + 2 + 2);
        let ix = solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &ctx.accounts.mint.key(),
            rent.minimum_balance(space),
            space as u64,
            &spl_token_2022::id(),
        );

        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        // Add closing authority
        Self::add_closing_authority(
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key(),
        )?;

        // Add permanent delegate
        Self::add_permanent_delegate(
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key()
        )?;

        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            // TODO: who should have authority here
            ctx.accounts.program_delegate.key(),
            ctx.accounts.token_metadata.key(),
        )?;

        // Initialize the actual mint data
        initialize_mint(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.rent,
            &ctx.accounts.token22_program.key(),
            // TODO incorrect
            &ctx.accounts.payer.key(),
            // TODO incorrect
            &ctx.accounts.payer.key(),
        )?;

        let now = Clock::get().unwrap().unix_timestamp;
        let destroy_timestamp = now
            .checked_add(params.destroy_timestamp_offset)
            .ok_or(EphemeralityError::InvalidCalculation)
            .unwrap();

        let token_metadata = &mut ctx.accounts.token_metadata;
        **token_metadata = EphemeralMetadata {
            update_authority: Some(*ctx.accounts.payer.key),
            name: params.name,
            symbol: params.symbol,
            uri: params.uri,
            mint: *ctx.accounts.mint.key,
            additional_metadata: vec![[EXPIRY_FIELD.to_string(), destroy_timestamp.to_string()]]
        };

        // TODO need to validate proper creation of everything
        // validate extensions are there and that metadata is created

        msg!("Wrote state {:?}", ctx.accounts.token_metadata);

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

        Ok(())
    }


    fn add_closing_authority(
        mint_account: &AccountInfo,
        program: Pubkey,
        program_delegate: Pubkey
    ) -> Result<()> {
        let ix = spl_token_2022::instruction::initialize_mint_close_authority(
            &program,
            &mint_account.key(),
            Some(&program_delegate),
        )?;

        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info(),
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }

    fn add_permanent_delegate(
        mint_account: &AccountInfo,
        program: Pubkey,
        program_delegate: Pubkey
    ) -> Result<()> {
        let ix = spl_token_2022::instruction::initialize_permanent_delegate(
            &program,
            &mint_account.key(),
            &program_delegate,
        )?;

        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info()
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }
}
