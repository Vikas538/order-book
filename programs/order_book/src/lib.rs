use anchor_lang::prelude::*;

declare_id!("Bn1tuFNWct5qTShqqQYd6hegr1QDBRRRLfH1xpaxorNd");

mod instructions;
use instructions::*;

mod state;


#[program]
pub mod order_book {
    use super::*;
    use crate::instructions::{buy_order::{BuyOrderLimit,buy_order},init_market::{InitializeMarket,initilize_market},match_order::{MatchOrder,match_order}};

    pub fn init_market(ctx: Context<InitializeMarket>,base_mint:Pubkey,quote_mint:Pubkey)->Result<()> {
        initilize_market(ctx,base_mint,quote_mint)
    }

    pub fn limit_buy_order(ctx: Context<BuyOrderLimit>, id: u64,quantity: u64,buy_price: u64,base_mint: Option<Pubkey>,quote_mint: Option<Pubkey>) -> Result<()> {
        buy_order(ctx, id,quantity,buy_price,base_mint,quote_mint)
    }

    pub fn limit_sell_order(ctx: Context<SellOrderLimit>, id: u64,quantity: u64,sell_price: u64,base_mint: Option<Pubkey>,quote_mint: Option<Pubkey>) -> Result<()> {
        sell_order(ctx, id,quantity,sell_price,base_mint,quote_mint)
    }

    pub fn make_transfer(ctx: Context<MatchOrder>,buy_order_id:u64,sell_order_id:u64)->Result<()> {
        match_order(ctx,buy_order_id,sell_order_id)
    }
}
