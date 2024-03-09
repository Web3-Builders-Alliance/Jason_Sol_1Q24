use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub seed: u64,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl Space for Escrow {const INIT_SPACE: usize = 8 + 32 + 8 + 32*2 + 8 + 1;}