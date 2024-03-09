use anchor_lang::prelude::*;

declare_id!("cTZHHALQnQyXJYTHK93AaWqEb5P3e6DHquCNo5Jricf");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
