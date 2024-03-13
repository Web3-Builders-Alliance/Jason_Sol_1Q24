use crate::{assert_non_zero, assert_not_expired, assert_not_locked};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
     token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::state::Config;
use crate::error::AmmError;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
 
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = auth
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: InterfaceAccount<'info, TokenAccount>,

    #[account(seeds = [b"auth"], bump)]
    /// CHECK: This is safe because it's just used to sign
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump, 
    )]
    pub config: Account<'info, Config>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64, expiration: i64) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([amount]);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        msg!("curve: {:?}", curve);

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        msg!("p: {:?}", p);

        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        assert_non_zero!([res.deposit, res.withdraw]);
        self.deposit_token(is_x, res.deposit)?;
        self.withdraw_token(is_x, res.withdraw)?;
        Ok(())
    }


    pub fn withdraw_token(&self, is_x: bool, amount:u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_y.to_account_info(), 
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals),
            false => (
                self.user_x.to_account_info(), 
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals),
        };
        let cpi_account = TransferChecked{
            from,
            to,
            authority: self.auth.to_account_info(),
            mint,
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_account, signer_seeds);
        transfer_checked(cpi_ctx, amount, decimals)
    }

    pub fn deposit_token(&self, is_x: bool, amount:u64) ->Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_x.to_account_info(), 
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals),
            false => (
                self.user_y.to_account_info(), 
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals),
        };
        let cpi_account = TransferChecked{
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };


        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_account);
        transfer_checked(cpi_ctx, amount, decimals)
    }
    
}