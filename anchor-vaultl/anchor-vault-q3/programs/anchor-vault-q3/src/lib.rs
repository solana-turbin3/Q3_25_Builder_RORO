#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("7Ax6cBnHWXeHyfno4Y1vAJPEgiuZfsaJR4cmcbaKdCRP");

#[program]
pub mod anchor_vault_q3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;
        
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        msg!("Deposited {} tokens", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        msg!("Withdrew {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
#[derive(Space)] //is used to define the size of the account
// same than the macro upper 
//impl Space for VaultState {
//    const INIT_SPACE: usize = 8 + 1 + 1;
//}

pub struct VaultState {
    pub vault_bump: u8, 
    pub state_bump: u8,
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"vault", user.key().as_ref()],
        bump, 
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut, 
        seeds = [b"vault", vault_state.key().as_ref()],
        bump, 
    )]
    pub vault: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: InitializeBumps) -> Result<()> {
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(), 
        }

        let cpi_ctx:  CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, rent_exempt)?;

        self.vault_state.vault_bump = bumps_vault;
        self.vault_state.state_bump = bumps.vault_state;

        msg!("Vault initialized");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"vault", user.key().as_ref()], bump = vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,

    #[account(mut, seeds = [b"state", user.key().as_ref()], bump = vault_state.state_bump)]

    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'Info> Deposit<'Info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(), 
        }

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        msg!("Deposited {} tokens", amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut, 
        seeds = [b"state", user.key().as_ref()], 
        bump = vault_state.vault_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut, 
        seeds = [b"vault", vault_state.key().as_ref()], 
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>, 
}

impl<'Info> Withdraw<'Info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        // CHeck that the withdraw leaves the vault with a rent exemplt balance

        // Check the account has enough funds for the user to withdraw

        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(), 
        }

        let seeds: &[&[&[u8]]] = &[&[
            b"vault", 
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump], 
        ];

        let signer_seeds: &[&[&[u8]]] = &[seeds, &[&[seeds[...]];

        let cpi_ctx: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        msg!("Withdrew {} tokens", amount);

        Ok(())
    }
}

