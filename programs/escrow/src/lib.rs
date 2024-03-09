use anchor_lang::prelude::*;

pub mod state;
pub mod context;
use context::*;
declare_id!("ExaetF9U4ujkqaUsn2imrmE6odyLBTc9YemUoMZDERu2");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_escrow(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {
        // let bumps = ctx.bumps;
        ctx.accounts.deposit(amount)?;
        ctx.accounts.make(&ctx.bumps, seed, amount)
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