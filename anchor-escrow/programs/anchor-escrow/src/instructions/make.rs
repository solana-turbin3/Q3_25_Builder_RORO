use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use create::Escrow;



#[derive(Accounts)]
#[instruction(seed: u64)
]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,

    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Interface<'info, SystemInterface>,
    pub associated_token_program: Interface<'info, AssociatedTokenInterface>,
   // pub rent: Interface<'info, RentInterface>,
}

impl<'info> Make<'info> {
    pub fn init_escrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
     
        self.escrow.set_inner(Escrow {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive,
            bump: bumps.escrow,
        });
     
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {

        let transfer_accounts: TransferChecked<'info> = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(program: self.token_program.to_account_info(), accounts: transfer_accounts);

        transfer_checked(cpi_ctx, amount: deposit, self.mint_a.decimals)?;
        Ok(())
    }