use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AdminAccount {
  pub bump: u8,
  pub authority: Pubkey
}