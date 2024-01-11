use ephemeral_metadata::TokenMetadata;
use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // TODO how to get exact space of this?
    #[account(
        seeds = [
            SEED_TOKEN_METADATA,
            mint.key().as_ref()
        ],
        seeds::program = ephemeral_metadata::ID.key(),
        bump
    )]
    pub token_metadata: Account<'info, TokenMetadata>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {
    pub destroy_timestamp_offset: i64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenCreateParams) -> Result<()> {
       
       Self::execute(
        ctx.accounts.mint.to_account_info().clone(),
        ctx.accounts.program_delegate.to_account_info().clone(),
        ctx.accounts.payer.to_account_info().clone(),
        ctx.accounts.rent.to_account_info().clone(),
        ctx.accounts.token22_program.to_account_info().clone(),
        &[ExtensionType::MetadataPointer]
       )?;

        Ok(())
    }

    pub fn execute<'info> (
        mint: AccountInfo<'info>,
        program_delegate: AccountInfo<'info>,
        payer: AccountInfo<'info>,
        rent_account: AccountInfo<'info>,
        token22_program: AccountInfo<'info>,
        additional_extensions: &[ExtensionType]
    ) -> Result<()> {

        // Initialise Mint Account
        Self::init_mint_account(
            rent_account.to_account_info(),
            payer.to_account_info(),
            mint.to_account_info(),
            additional_extensions
        )?;

        // Add closing authority
        Self::add_closing_authority(
            &mint,
            token22_program.key(),
            program_delegate.key(),
        )?;

        // Add permanent delegate
        Self::add_permanent_delegate(
            &mint.to_account_info(),
            token22_program.key(),
            program_delegate.key()
        )?;
        
        // Initialize the actual mint data
        initialize_mint(
            &mint.to_account_info(),
            &rent_account.to_account_info(),
            &token22_program.key(),
            // TODO incorrect
            &payer.key(),
            // TODO incorrect
            &payer.key(),
        )?;

        Ok(())
    }

    fn init_mint_account<'info> (
        rent_account: AccountInfo<'info>,
        payer: AccountInfo<'info>,
        mint: AccountInfo<'info>,
        additional_extensions: &[ExtensionType]
    ) -> Result<()> {

        // standard extensions
        let mut extensions = vec![
            ExtensionType::PermanentDelegate,
            ExtensionType::MintCloseAuthority
        ];

        extensions.extend_from_slice(additional_extensions);

        // calculate extension sizes
        let extension_sizes = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&extensions).unwrap();

        let rent = &Rent::from_account_info(&rent_account)?;
        // TODO need to have collectionConfig passed in

        // TODO: all NFTs should have same expiration date upon mint
        // maybe just save the now date and the destroytimeoffset

        // TODO: need to calculate this properly
        let space = extension_sizes + (64 + 2 + 2);
        let ix = solana_program::system_instruction::create_account(
            &payer.key(),
            &mint.key(),
            rent.minimum_balance(space),
            space as u64,
            &spl_token_2022::id(),
        );

        let account_infos: Vec<AccountInfo> = vec![
            payer,
            mint
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
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
