use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SensorReading {
    pub authority: Pubkey,
    pub bump: u8,
    pub pm25: u16,
    pub pm10: u16,
    pub temperature: u16,
    pub humidity: u16,
    pub timestamp: u64,
}
