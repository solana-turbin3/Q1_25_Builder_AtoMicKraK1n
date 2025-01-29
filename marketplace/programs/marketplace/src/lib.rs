use anchor_lang::prelude::*;

declare_id!("BURqqkuZppLd47LPQJEsRdz7rvDikGcXfGL2tMHDRuS2");

mod state;
mod errors;

mod contexts;
use errors::*;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
