use anchor_lang::{prelude::*, solana_program::{ed25519_program, sysvar::instructions::load_instruction_at_checked}, system_program::{transfer, Transfer}};
use crate::{state::Bet};


pub const HOUSE_EDGE: u16 = 150;

#[derive(Accounts)]
pub struct ResolveBet <'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(mut)]
    ///CHECK: this is ok
    pub player: UncheckedAccount<'info>,
    #[account(
        mut, 
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut, 
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    ///CHECK: this is safe
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

impl <'info> ResolveBet <'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        require_keys_eq!(ix.program_id, ed25519_program::ID); //  <-- Add error
        require_eq!(ix.accounts.len(), 0); // <-- add error
        let signature = Ed25519InstructionSignature::unpack(&ix.data)?.0;
        require_eq!(signatures.len(), 1); // <-- add error
        require!(signature.isverifiable); // <-- error
        let signature = &signature[0];
        require!(signature.is_verifiable); // error
        require_keys_eq!(signature.public_key.ok?, self.house.key());
        require!(&signature.signature.ok)?.eq(sig);
        // require!(&signature)
        unimplemented!()
    }

    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, sig: &[u8]) -> Result<()> {
        unimplemented!()
    }
}




