use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
pub use crate::state::*;

#[derive(Accounts)]
pub struct Revoke<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault_keeper: SystemAccount<'info>,

    #[account(mut, 
        has_one = employer,
        constraint = vault_keeper.key() == vault_state.vault_keeper,
        close = employer,
        seeds = [b"vault_state", vault_state.seed.to_le_bytes().as_ref(), vault_state.employer.as_ref()],//, vault_state.worker.as_ref()], 
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, ContractAccount>,
    pub system_program: Program<'info, System>,
}

impl<'info> Revoke<'info> { 
    pub fn revoke_contract(&mut self) -> Result<()> {

    let vault_state = self.vault_state.key();    
    let tr_accounts = Transfer {
        from: self.vault_keeper.to_account_info(),
        to: self.employer.to_account_info(),
    };
    
    
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
