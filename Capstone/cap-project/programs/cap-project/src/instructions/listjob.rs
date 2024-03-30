use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
pub use crate::state::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            b"vault",
            vault_state.key().as_ref(),
        ],
        bump
    )]
    pub vault_keeper: SystemAccount<'info>,
    
    #[account(init,
    payer = employer,
    seeds = [b"vault_state", seed.to_le_bytes().as_ref(), employer.key().as_ref()],
    bump, 
    space = ContractAccount::INIT_SPACE
    )]
    pub vault_state: Account<'info, ContractAccount>,
    pub system_program: Program<'info, System>,
}


impl<'info> Make<'info> {
    pub fn make_contract(&mut self,seed: u64,amount: u64,time_period: i64, ) -> Result<()> {

        self.vault_state.employer = self.employer.key();
        self.vault_state.worker = None;
        self.vault_state.seed = seed;
        self.vault_state.state_bump;
        self.vault_state.vault_keeper = self.vault_keeper.key();
        self.vault_state.vault_bump;
        self.vault_state.created_at = Clock::get()?.unix_timestamp;
        self.vault_state.amount = amount;
        self.vault_state.lock_seconds = time_period;

        Ok(())
    }


    pub fn transfer_amount(&mut self, amount: u64) -> Result<()> {
        let tr_accounts = Transfer {
            from: self.employer.to_account_info(),
            to: self.vault_keeper.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.system_program.to_account_info(), tr_accounts);
        transfer(cpi_context, amount)
    }



}

