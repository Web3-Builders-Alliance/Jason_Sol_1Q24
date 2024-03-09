use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{
        Mint, 
        Token,
        TokenAccount,
        Transfer,
        transfer
}};

use crate::state::Escrow;


#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,
    pub maker: UncheckedAccount<'info>,
    #[account(
        mut, 
        seeds = [
            b"escrow",
            maker.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = donor, //consider initialize the account in the make context so that the maker can pay the init fees
        associated_token::mint = mint,
        associated_token::authority= escrow,
    )]
    pub escrow_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = donor,
    )]
    pub donor_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Donate <'info> {
    pub fn donate(&mut self, amount: u64) -> Result<()> {
        let total_donate = self.escrow_ata.amount;
        let remaining = self.escrow.target - total_donate;

        let amount_to_transfer = match amount > remaining {
            true => remaining,
            false => amount,
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.donor.to_account_info(),
            to: self.escrow_ata.to_account_info(),
            authority: self.donor.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount_to_transfer)?;

        msg!("Donation {} token successful", amount_to_transfer);
        msg!("Total donated: {}", self.escrow_ata.amount);

        Ok(())
    }

    pub fn check_donations(&self) -> Result<()> {
        match self.escrow_ata.amount >= self.escrow.target {
            true => {
                
                let seeds = &[
                    b"escrow",
                    self.maker.key.as_ref(),
                    self.mint.key.as_ref(),
                    &[self.escrow.bump],
                ];
                let signer_seeds = &[&[seeds[..]]];

                let cpi_program = self.token_program.to_account_info();

                let cpi_accounts = Transfer {
                    from: self.escrow_ata.to_account_info(),
                    to: self.maker_ata.to_account_info(),
                    authority: self.escrow.to_account_info(),
                };
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
                transfer(cpi_ctx, self.escrow_ata.amount())?;

            }
            false => msg!("The escrow has not reached the target yet")
        }
        Ok(())
    }
    
}