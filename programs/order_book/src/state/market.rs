use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market{
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub total_orders_created:u64,
    pub bump:u8,
}