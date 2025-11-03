use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("GHZ29nUjyQrLcfxFNUTtKbLxmrkW3Za7ysk9Xi6yofsc");

#[program]
pub mod vault_savings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.initialize(amount, &ctx.bumps)?;

        Ok(())
    }
    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }
    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, amount: u64, bumps: &InitializeBumps) -> Result<()> {
        self.state.amount = amount;
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state".as_ref(), user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl<'info> Operations<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        self.check_balance()?;

        Ok(())
    }

    pub fn check_balance(&mut self) -> Result<()> {
        if self.vault.lamports() >= self.state.amount {
            let cpi_program = self.system_program.to_account_info();
            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            };

            let seeds = &[
                b"vault".as_ref(),
                self.state.to_account_info().key.as_ref(),
                &[self.state.vault_bump],
            ];

            let signature_seeds = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signature_seeds);
            transfer(cpi_ctx, self.vault.lamports())?;

            self.state.close(self.user.to_account_info())?;
        }
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault".as_ref(),
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signature_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signature_seeds);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[account]
pub struct VaultState {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    // anchor + amount + vault bump+ state bump
    const INIT_SPACE: usize = 8 + 8 + 1 + 1;
}
