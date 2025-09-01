use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::{anchor::delegate, cpi::DelegateConfig};

#[delegate]
#[derive(Accounts)]
pub struct DelegateReading<'info> {
    pub payer: Signer<'info>,
    /// CHECK: The pda to delegate
    #[account(mut, del)]
    pub sensor_reading: AccountInfo<'info>,
}

impl<'info> DelegateReading<'info> {
    pub fn delegate(&mut self) -> Result<()> {
      self.delegate_sensor_reading(
        &self.payer,
        &[b"sensor_reading", &self.payer.key().as_ref()], 
        DelegateConfig::default(),
      )?;
      Ok(())
    }
}