use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
pub use crate::error::ErrorMessages;

use crate::{ApplicationState, ContractAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub worker: Signer<'info>,
    
    #[account(mut)]
    pub employer: SystemAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref(),],
        bump = vault_state.vault_bump,
    )]
    pub vault_keeper: SystemAccount<'info>,
    
    #[account(mut, 
        constraint = worker.key() == vault_state.worker.unwrap(),
        constraint = vault_keeper.key() == vault_state.vault_keeper,
        constraint = vault_state.created_at + vault_state.lock_seconds < Clock::get()?.unix_timestamp @ ErrorMessages::VaultNotExpired,
        close = employer,
        seeds = [b"vault_state",vault_state.seed.to_le_bytes().as_ref(),vault_state.employer.as_ref(),vault_state.worker.unwrap().as_ref()], 
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, ContractAccount>,
    pub application: Account<'info, ApplicationState>,
    pub system_program: Program<'info, System>,
}


impl<'info> Claim<'info> { 
    pub fn claim(&mut self) -> Result<()> {
        let tr_accounts = Transfer {
            from: self.vault_keeper.to_account_info(),
            to: self.worker.to_account_info(),
        };
    
        let vault_state = self.vault_state.key();
        let seeds = &[
            "vault".as_bytes(),
            vault_state.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];


        let cpi_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(), 
            tr_accounts, 
            signer_seeds);
        transfer(cpi_context, self.vault_keeper.get_lamports())
    }
}