pub mod refund;
pub mod make; 
pub use refund::*;
pub use make::*;

use anchor_spl::token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked};
use create::Escrow;


#[derive(Accounts)]
#[instruction(seed: u64)
]
pub struct Refund<'info> {
  #[account(mut)]
  pub maker: Signer<'info>,

  #[account(
      mint::token_program = token_program,
  )]
  pub mint_a: InterfaceAccount<'info, Mint>,

  #[account(
      mut, 
      associated_token::mint = mint_a,
      associated_token::authority = maker,
      associated_token::token_program = token_program,

  )]
  pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

  #[account(
      init,
      close = maker,
      has_one = mint_a,
      has_one = maker,
      payer = maker,
      space = 8 + Escrow::INIT_SPACE,
      seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump,
  )]
  pub escrow: Account<'info, Escrow>,

  #[account(
      mut,
      associated_token::mint = mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program,
  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,

  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Interface<'info, SystemInterface>,
  pub associated_token_program: Interface<'info, AssociatedTokenInterface>,
    
} 

impl<'info> Refund<'info> {
    pub fn refund_and_close(&mut self) -> Result<()> {
      let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow", 
        self.escrow.maker.as_ref(), 
        self.escrow.seed.to_le_bytes().as_ref(),
        &[self.escrow.bump],
      ]];

      let transfer_accounts: TransferChecked<'info> = TransferChecked { 
        from: self.vault.to_account_info(),
        mint: self.mint_a.to_account_info(),
        to: self.maker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
      };

        let transfer_cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(program: self.token_program.to_account_info(), accounts: transfer_accounts, signer_seeds: &signer_seeds);

        transfer_checked(transfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let close_accounts: CloseAccount<'info> = CloseAccount {
            account: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new_with_signer(program: self.token_program.to_account_info(), accounts: close_accounts, signer_seeds: &signer_seeds);
        close_account(close_cpi_ctx)?;

        Ok(())
    }
}