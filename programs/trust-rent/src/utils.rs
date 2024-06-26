use super::payments::{PayRent, Payment};

use anchor_lang::prelude::*;
use solana_program::pubkey;

pub const USDC_MINT_ADDRESS: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const DEV_USDC_ADDRESS: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

impl<'info> PayRent<'info> {
    pub fn payment_details(&mut self, rent: u64) -> Result<Payment> {
        let clock = Clock::get()?;
        let payment_date = clock.unix_timestamp;
        let payment = Payment {
            payment_id: 0,
            amount: rent,
            date: payment_date as u64,
        };
        Ok(payment)
    }
}
