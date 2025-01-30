use anchor_lang::prelude::*;

declare_id!("GkiKqSVfnU2y4TeUW7up2JS9Z8g1yjGYJ8x2QNf4K6Y");

mod state;
mod errors;

mod contexts;
use contexts::*;
use errors::*;

#[program]
pub mod anchor_marketplace {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
    }

    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;

        Ok(())
    }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     ctx.accounts.send_sol()?;
    //     ctx.accounts.send_nft()?;
    //     ctx.accounts.close_mint_vault()?;

    //     Ok(())
    // }
}