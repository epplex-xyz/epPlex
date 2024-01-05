use crate::*;


#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    // #[account(
    //     init,
    //     payer = payer,
    //     space = Mint::LEN,
    // )]
    // /// CHECK
    // pub mint: Box<InterfaceAccount<'info, MintInterface>>,
    #[account(mut)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

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

    pub fn actuate(ctx: Context<Self>, params: TokenCreateParams) -> Result<()> {3
        // TODO add mint account creation within IX

        // Has to be similar to the old create Ticket account

        // extended_mint.rs, at line 44
        let extension_sizes = ExtensionType::try_calculate_account_len::<Mint>(
            &[ExtensionType::PermanentDelegate, ExtensionType::MintCloseAuthority]
        ).unwrap();

        let rent = &Rent::from_account_info(&ctx.accounts.rent.to_account_info())?;
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

        // Self::create_mint_account(
        //     &ctx.accounts.mint,
        //     ctx.accounts.token22_program.key(),
        //     ctx.accounts.program_delegate.key(),
        // )?;

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
            &ctx.accounts.payer,
            // &ctx.accounts.mint,
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.payer,
            params.name,
            params.symbol,
            params.uri,
        )?;

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

        Ok(())
    }

    // fn create_mint_account(
    //     mint_account: &AccountInfo,
    //     program: Pubkey,
    //     program_delegate: Pubkey
    // ) -> Result<()> {
    //     // Has to be similar to the old create Ticket account
    //
    //     // extended_mint.rs, at line 44
    //     let extension_sizes = ExtensionType::try_calculate_account_len::<Mint>(
    //         &[ExtensionType::PermanentDelegate, ExtensionType::MintCloseAuthority]
    //     ).unwrap();
    //
    //     let rent = &Rent::from_account_info(rent_sysvar_info)?;
    //     let space = extension_sizes + (64 + 2 + 2);
    //     let ix = solana_program::system_instruction::create_account(
    //         &ctx.payer.pubkey(),
    //         &mint_account.pubkey(),
    //         rent.minimum_balance(space),
    //         space as u64,
    //         &spl_token_2022::id(),
    //     );
    //
    //     let account_infos: Vec<AccountInfo> = vec![
    //         mint_account.to_account_info(),
    //     ];
    //
    //     solana_program::program::invoke(
    //         &ix,
    //         &account_infos[..],
    //     )?;
    //
    //     Ok(())
    // }

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
