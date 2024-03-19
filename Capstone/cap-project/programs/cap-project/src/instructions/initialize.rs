use anchor_lang::prelude::*;

pub use crate::state::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
