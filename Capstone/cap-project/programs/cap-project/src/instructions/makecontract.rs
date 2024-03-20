use anchor_lang::prelude::*;
use anchor_spl::token::Token;
pub use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,

    #[account(mut)]
    pub worker: AccountInfo<'info>,
    
    #[account(
        init,
        payer = employer,
        seeds = [b"contract", employer.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = ContractAccount::INIT_SPACE,
    )]
    pub contract: Account<'info, ContractAccount>,
    
    #[account(
        init,
        payer = employer,
        seeds = [b"contract_state", &seed.to_le_bytes()[..], &employer.key().to_bytes(), &worker.key().to_bytes()],
        bump,
        space = ContractAccount::INIT_SPACE
    )]
    pub contract_account: Account<'info, ContractAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


impl<'info> Make<'info> {
    pub fn make_contract(&mut self,seed: u64,amount: u64,time_period: i64, ) -> Result<()> {

        self.contract_account.seed = seed;
        self.contract_account.amount = amount;
        self.contract_account.state_bump;
        self.contract_account.employer = self.employer.key();
        self.contract_account.worker = self.worker.key();
        self.contract_account.created_at = Clock::get()?.unix_timestamp;
        self.contract_account.lock_seconds = time_period;

        Ok(())
    }
}


