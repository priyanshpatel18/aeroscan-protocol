use anchor_lang::prelude::*;

use crate::{errors::ErrorCode, states::SensorReading, SensorReadingEvent};

#[derive(Accounts)]
#[instruction(authority: Pubkey)]
pub struct UpdateReading<'info> {
    #[account(
        mut,
        seeds = [b"sensor_reading", authority.as_ref()],
        bump = sensor_reading.bump,
        has_one = authority
    )]
    pub sensor_reading: Account<'info, SensorReading>,
}

impl<'info> UpdateReading<'info> {
    pub fn update_reading(
        &mut self,
        _authority: Pubkey,
        pm25: u16,
        pm10: u16,
        temperature: u16,
        humidity: u16,
    ) -> Result<()> {
        require!(
            self.sensor_reading.authority == self.sensor_reading.authority.key(),
            ErrorCode::Unauthorized
        );

        let timestamp: u64 = Clock::get()?.unix_timestamp as u64;
        self.sensor_reading.pm25 = pm25;
        self.sensor_reading.pm10 = pm10;
        self.sensor_reading.temperature = temperature;
        self.sensor_reading.humidity = humidity;
        self.sensor_reading.timestamp = timestamp;

        emit!(SensorReadingEvent {
            pm25,
            pm10,
            temperature,
            humidity,
            timestamp,
        });

        Ok(())
    }
}
