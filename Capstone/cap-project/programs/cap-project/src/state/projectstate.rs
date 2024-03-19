use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16,
    pub name: String,
}

#[account]
pub struct UserAccount {
    pub user_pubkey: Pubkey,
    pub balance: u64,   
}

#[account]
pub struct ContractAccount {
    pub creator_pubkey: Pubkey,
    pub contract_pubkey: Pubkey,
    pub balance: u64,
    pub salary: u64,
    pub time_period: u64,
    pub unlock_conditions: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + (4 );
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 8 + 32 + 1;
}

impl Space for ContractAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 3 + (4 );
}
