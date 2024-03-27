use anchor_lang::prelude::*;


#[account]
pub struct ContractAccount {
    pub employer: Pubkey,
    pub worker: Option<Pubkey>,
    pub seed: u64,
    pub state_bump: u8,
    pub vault_keeper: Pubkey,
    pub vault_bump: u8,
    pub created_at: i64,
    pub amount: u64,
    pub lock_seconds: i64,
}

#[account]
pub struct ApplicationState {
    pub worker: Pubkey,
    pub listing: Pubkey,
    pub bump: u8,
}

impl Space for ApplicationState {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1;
}

impl Space for ContractAccount {
    const INIT_SPACE: usize = 8 + 32 + 33 + 8 + 1 + 32 + 1 + 8*3;
}
