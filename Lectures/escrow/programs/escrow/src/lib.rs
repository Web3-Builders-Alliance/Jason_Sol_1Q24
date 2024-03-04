use anchor_lang::prelude::*;

declare_id!("5tB5mvDUvgLWDmiSymUBCtvsu6jM21462iGWXbAUXhx1");

pub mod constants;
pub mod instructions;
pub mod state;
pub use instructions::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make_escrow(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {
        // let bumps = ctx.bumps;
        ctx.accounts.transfer(amount)?;
        ctx.accounts.make(seed, amount, &ctx.bumps)
    }

    pub fn refund_escrow(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }

    pub fn take_escrow(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.pay_back()?;
        ctx.accounts.take()
    }
}

#[derive(Accounts)]
pub struct Initialize {}