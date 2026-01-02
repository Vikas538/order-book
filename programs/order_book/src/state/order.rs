use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BuyOrder{
    pub id:u64,
    pub owner: Pubkey,
    pub quantity:u64,
    pub remaining:u64,
    pub quantity_filled:u64,
    pub buy_price:u64,
    pub is_filled:bool,
    pub base_mint: Option<Pubkey>,
    pub quote_mint: Option<Pubkey>,
    pub created_at:u64,
    pub bump:u8,
}

#[account]
#[derive(InitSpace)]
pub struct SellOrder{
    pub id:u64,
    pub owner: Pubkey,
    pub quantity:u64,
    pub quantity_filled:u64,
    pub remaining:u64,
    pub sell_price:u64,
    pub is_filled:bool,
    pub base_mint: Option<Pubkey>,
    pub quote_mint: Option<Pubkey>,
    pub created_at:u64,
    pub bump:u8,
}