use anchor_lang::prelude::*;

mod errors;
mod state;
mod instructions;

use instructions::*;

declare_id!("4GXgodsZ7on5FCtBT7v7Xoi3aXs49LhHRFAYAqGeoA7E");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
