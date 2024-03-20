pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("aNRwZCzkCWyMtA25PKdqL4JVm3rpmw6CQJ7QjpPUWSL");

#[program]
pub mod cap_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee)
    }

    pub fn make_contract(ctx: Context<Make>, seed: u64, amount: u64,time_period: i64) -> Result<()> { 
        ctx.accounts.make_contract(seed, amount, time_period)
    }
}
