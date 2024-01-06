use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    // TODO: is unchecked account correct?
    #[account(mut)]
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

    // #[account(
    //     init,
    //     seeds = [
    //         SEED_TOKEN_METADATA,
    //         ID,
    //         mint.key().as_ref()
    //     ],
    //     payer = payer,
    //     space = TokenMetadata::LEN
    //     bump,
    // )]
    // pub token_metadata: Account<'info, TokenMetadata>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {
    destroy_timestamp_offset: i64,
    name: String,
    symbol: String,
    uri: String,
}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenCreateParams) -> Result<()> {
        // Initialise Mint Account
        let extension_sizes = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &[ExtensionType::PermanentDelegate, ExtensionType::MintCloseAuthority]
        ).unwrap();
        let rent = &Rent::from_account_info(&ctx.accounts.rent.to_account_info())?;

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
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key(),
        )?;

        // Add permanent delegate
        Self::add_permanent_delegate(
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key()
        )?;

        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_delegate.key(),
            // TODO this address needs to change to separate PDA
            ctx.accounts.mint.key(),
        )?;

        // Initialize mint
        initialize_mint(
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.rent,
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.payer.key(),
            &ctx.accounts.payer.key(),
        )?;

        transfer_sol(
            &ctx.accounts.system_program,
            &ctx.accounts.payer,
            &ctx.accounts.mint.to_account_info(),
            // &ctx.accounts.mint,
            // TODO need to compute exact amount
            // 2000000 is OK
            1800000 // 0.0005 SOL
        )?;

        add_token_metadata(
            &ctx.accounts.token22_program.key(),
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.payer, // this needs to change
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.payer,
            params.name,
            params.symbol,
            params.uri,
        )?;

        // TODO this can be done better
        // https://github.com/solana-labs/solana-program-library/blob/f382e76c5c1be20be208ce54c32719e8f0a2f5e1/token/program-2022-test/tests/token_metadata_initialize.rs#L60
        let field = "destroyTimestamp";
        let now = Clock::get().unwrap().unix_timestamp;
        let destroy_timestamp = now
            .checked_add(params.destroy_timestamp_offset)
            .ok_or(EphemeralityError::InvalidCalculation)
            .unwrap();

        update_token_metadata(
            &ctx.accounts.token22_program.key(),
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.payer, // who is allowed to make changes here? Changes have to go through program?
            spl_token_metadata_interface::state::Field::Key(field.to_string()),
            destroy_timestamp.to_string(),
        )?;

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
