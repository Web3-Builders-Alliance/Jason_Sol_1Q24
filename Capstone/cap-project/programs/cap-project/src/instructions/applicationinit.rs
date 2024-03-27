use anchor_lang::prelude::*;
pub use crate::state::*;
pub use crate::error::ErrorMessages;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Applicate<'info> {
    #[account(mut)]
    pub worker: Signer<'info>,

    #[account(
        init,
        payer = worker,
        space = ApplicationState::INIT_SPACE,
        seeds = [b"application", worker.key().as_ref(), joblisting.key().as_ref()],
        bump,
    )]
    pub application: Account<'info, ApplicationState>,
    pub joblisting: Account<'info, ContractAccount>,
    pub system_program: Program<'info, System>,

}


impl<'info> Applicate<'info> {
    pub fn create_application(&mut self) -> Result<()> {
        require_eq!(self.application.listing, self.joblisting.key(), ErrorMessages::Error3);

        self.application.worker = self.worker.key();

        Ok(())
    }
}
