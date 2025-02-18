use anchor_lang::prelude::*;

declare_id!("B3eJtrwzFYYT4ifWkro3RPNwwbXbgaZpDRGbvwAfmqzo");

pub mod contexts;
pub mod state;
pub mod error;

pub use contexts::*;
pub use state::*;
pub use error::*;

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)?;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, amount: u64, roll: u8) -> Result<()> {
        ctx.accounts.create_bet(seed, roll, amount, &ctx.bumps)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)
    }

}