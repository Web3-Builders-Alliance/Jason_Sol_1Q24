use anchor_lang::prelude::*;

pub mod state;
pub mod context;
use context::*;

declare_id!("cTZHHALQnQyXJYTHK93AaWqEb5P3e6DHquCNo5Jricf");


#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, amount: u64) -> Result<()> {
        ctx.accounts.make(amount, &ctx.bumps)?;
        Ok(())
    }
}


