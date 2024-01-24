use crate::*;

#[derive(Accounts)]
pub struct TokenBuy<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,


    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = buyer_token_account.mint,
        associated_token::authority = buyer,
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,


    // TODO needs to be a different type
    #[account(mut)]
    /// CHECK
    pub seller: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = seller_token_account.mint,
        associated_token::authority = seller,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    pub rent: Sysvar<'info, Rent>,
    // TODO is systemprogram required?
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBuyParams {}

impl TokenBuy<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenBuyParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenBuyParams) -> Result<()> {

        //Transfer price
        // let ix = solana_program::system_instruction::transfer(
        //     &ctx.accounts.buyer.key(),
        //     &ctx.accounts.seller.key(),
        //     ctx.accounts.token_metadata.price,
        // );
        //
        // let account_infos: Vec<AccountInfo> = vec![
        //     ctx.accounts.buyer.to_account_info(),
        //     ctx.accounts.seller.to_account_info(),
        //     ctx.accounts.system_program.to_account_info(),
        // ];
        //
        // solana_program::program::invoke(
        //     &ix,
        //     &account_infos[..],
        // )?;
        //
        //
        // //Create
        // anchor_spl::associated_token::create(
        //     CpiContext::new(
        //         ctx.accounts.token22_program.to_account_info(),
        //         anchor_spl::associated_token::Create {
        //             payer: ctx.accounts.buyer.to_account_info(), // payer
        //             associated_token: ctx.accounts.ata_buyer.to_account_info(),
        //             authority: ctx.accounts.buyer.to_account_info(), // owner
        //             mint: ctx.accounts.mint.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token22_program.to_account_info(),
        //         }
        //     ),
        // )?;
        //
        //

        transfer_token_with_pda(
            1,
            0,
            &ctx.accounts.token22_program,
            &ctx.accounts.seller_token_account.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.buyer_token_account.to_account_info(),
            &ctx.accounts.permanent_delegate.to_account_info(),
            &[
                &ctx.accounts.permanent_delegate.key()
            ]
        )?;

        //
        // // disable listing
        // let metadata = &mut ctx.accounts.token_metadata;
        // metadata.for_sale = false;
        // metadata.price = 0;
        //

        Ok(())
    }

}
