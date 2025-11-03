use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
use state::*;
declare_id!("GHZ29nUjyQrLcfxFNUTtKbLxmrkW3Za7ysk9Xi6yofsc");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, recive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }
    pub fn take(ctx: Context<Make>, seed: u64, deposit: u64, recive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, recive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
