use anchor_lang::prelude::*;

declare_id!("DtBhUvHUGQvnugY8kAwXh4bNBJ5GsAKcTh49uJwyKTcz");

pub mod contexts;
pub mod state;
pub mod error;

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
