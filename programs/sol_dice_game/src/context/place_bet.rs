use anchor_lang::{prelude::*, solana_program::address_lookup_table::instruction, system_program::{transfer, Transfer}};
use crate::{state::Bet};

#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct PlaceBet <'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    /// CHECK: this is safe
    pub house: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = Bet::LEN,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet <'info> {
    pub fn create_bet(&mut self, bumps: &PlaceBetBumps, seed: u128, roll: u8, amount: u64) -> Result<()> {
        self.bet.slot = Clock::get()?.slot;
        self.bet.player = self.player.key();
        self.bet.seed = seed;
        self.bet.amount = amount;
        self.bet.roll = roll;
        self.bet.bump = bumps.bet;
        Ok(())
    }

    pub fn deposit(&mut self, amount:u64) -> Result<()> {
        let account = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info()
        };
        
        let ctx = CpiContext::new(self.system_program.to_account_info(), account);
        transfer(ctx, amount)

    }




}
