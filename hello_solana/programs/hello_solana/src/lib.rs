use anchor_lang::prelude::*;

declare_id!("EJpfXPJnZoj7oj32ZLrdokb8FSGSt2NmYtukvVJVWmYU");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Solana");
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
