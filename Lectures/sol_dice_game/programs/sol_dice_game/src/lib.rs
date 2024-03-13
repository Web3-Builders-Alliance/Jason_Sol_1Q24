use anchor_lang::prelude::*;
mod state;
// mod error;
mod context;

use context::*;
declare_id!("7jVa32ZXzsxseS1YJfh4wUmAkTXetmRCTHAF4yMvHFdT");

#[program]
pub mod sol_dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}


