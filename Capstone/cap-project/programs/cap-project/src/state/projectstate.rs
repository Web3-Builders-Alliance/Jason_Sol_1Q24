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
    pub bump: u8,
    pub balance: u64,   
}

#[account]
pub struct ContractAccount {
    pub employer: Pubkey,
    pub worker: Pubkey,
    pub contract: Pubkey,
    pub seed: u64,
    pub state_bump: u8,
    pub contract_bump: u8,
    pub created_at: i64,
    pub amount: u64,
    pub lock_seconds: i64,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + (4);
}

impl Space for UserAccount {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1;
}

impl Space for ContractAccount {
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 1 + 1 + 8 + 8 + 8;
}



// #[account]
// pub struct ContractAccount {
//     pub employer: Pubkey,
//     pub contract: Pubkey,
//     pub worker: Pubkey,
//     pub bump:  u8,
//     pub balance: u64,
//     pub salary: u64,
//     pub time_period: u64,  // ?????
//     // pub unlock_conditions: String,
// }
// impl Space for ContractAccount {
//     const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 1 + 3 ;//+ (4);
// }
