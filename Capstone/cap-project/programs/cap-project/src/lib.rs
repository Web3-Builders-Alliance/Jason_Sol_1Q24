pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("aNRwZCzkCWyMtA25PKdqL4JVm3rpmw6CQJ7QjpPUWSL");

#[program]
pub mod cap_project {
    use super::*;

    pub fn make_contract(ctx: Context<Make>, seed: u64, amount: u64,time_period: i64) -> Result<()> { 
        ctx.accounts.make_contract(seed, amount, time_period)?;
        ctx.accounts.transfer_amount(amount)
    }

    pub fn revoke_contract(ctx: Context<Revoke>)-> Result<()> { 
        ctx.accounts.revoke_contract()
    }

    pub fn application(ctx: Context<Applicate>) -> Result<()> {
        ctx.accounts.create_application()
    }

    pub fn acceptapplication(ctx: Context<AcceptApplication>) -> Result<()> {
        ctx.accounts.accept_application()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }


}
