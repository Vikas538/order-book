use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market{
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub bump:u8,
}