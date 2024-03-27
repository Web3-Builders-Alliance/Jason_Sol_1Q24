use anchor_lang::prelude::*;
pub use crate::state::*;
pub use crate::error::ErrorMessages;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct AcceptApplication<'info> {
    #[account(mut)]
    pub employer: Signer<'info>,
    
    #[account(
        init,
        payer = employer,
        space = ApplicationState::INIT_SPACE,
        seeds = [b"application", employer.key().as_ref(), joblisting.key().as_ref()],
        bump,
    )]
    pub application: Account<'info, ApplicationState>,
    pub joblisting: Account<'info, ContractAccount>,
    pub system_program: Program<'info, System>,

}


impl<'info> AcceptApplication<'info> {
    pub fn accept_application(&mut self) -> Result<()> {
        require_eq!(self.application.listing, self.joblisting.key(), ErrorMessages::Error1);
        require!(self.joblisting.worker.is_none(), ErrorMessages::Error2);
        self.joblisting.worker = Some(self.application.worker.key());
        Ok(())
    }
}
