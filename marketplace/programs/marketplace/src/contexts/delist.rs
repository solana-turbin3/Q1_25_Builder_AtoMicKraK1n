use anchor_lang::prelude::*;
use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    marketplace: Account<'info, Marketplace>,
    maker_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    listing: Account<'info, Listing>,
    #[account(mut)]
    vault: InterfaceAccount<'info, TokenAccount>,
}