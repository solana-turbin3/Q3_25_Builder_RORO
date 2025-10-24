use anchor_lang::prelude::*;

use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use create::Escrow;


#[derive(Accounts)]

pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        payer = taker
        init_if_needed, 
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed, 
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        associated_token::authority = escrow
        associated_token::mint = mint_a,
        associated_token::token_program = token_program,
    )]

        pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, 
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]

    pub escrow: Account<'info, Escrow>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Interface<'info, SystemInterface>,
    pub associated_token_program: Interface<'info, AssociatedTokenInterface>,
    pub rent: Interface<'info, RentInterface>,
    
}

impl Take<'_> {
    pub fn deposit(&mut self) -> Result<()> {

      let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();

        let cpi_accounts: TransferChecked<'info> = TransferChecked {
            from: self.taker_ata_a.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.taker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.vault.amount, self.mint_b.decimals)?;
        Ok(())
    }
}