use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken,  token_interface::{Mint,TokenAccount,TokenInterface,transfer_checked,TransferChecked}};
use crate::state::{Market,SellOrder};
use crate::state::errors::ErrorCode;

pub fn sell_order(ctx:Context<SellOrderLimit>,id: u64,quantity: u64,sell_price: u64,base_mint: Option<Pubkey>,quote_mint: Option<Pubkey>) -> Result<()> {

    ctx.accounts.sell_order.id = id;
    ctx.accounts.sell_order.owner = ctx.accounts.user.key();
    ctx.accounts.sell_order.quantity = quantity;
    ctx.accounts.sell_order.remaining = quantity;
    ctx.accounts.sell_order.sell_price = sell_price;
    ctx.accounts.sell_order.quantity_filled = 0;
    ctx.accounts.sell_order.is_filled = false;
    ctx.accounts.sell_order.base_mint = base_mint;
    ctx.accounts.sell_order.quote_mint = quote_mint;
    ctx.accounts.sell_order.created_at = Clock::get()?.unix_timestamp as u64;
    ctx.accounts.sell_order.bump = ctx.bumps.sell_order;
    ctx.accounts.market.total_orders_created += 1;
    msg!("Sell order created successfully");

  

    transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked{
        from:ctx.accounts.user_token_account.to_account_info(),
        to:ctx.accounts.base_vault.to_account_info(),
        authority:ctx.accounts.user.to_account_info(),
        mint:ctx.accounts.token_mint.to_account_info(),
    }), quantity, ctx.accounts.token_mint.decimals)?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(id:u64,base_mint:Pubkey,quote_mint:Pubkey)]

pub struct SellOrderLimit<'info>{

    #[account(mut)]
    pub user:Signer<'info>,

    #[account(init,payer = user, space = 8 + SellOrder::INIT_SPACE,seeds =[b"sell_limit_order",id.to_le_bytes().as_ref()],bump)]
    pub sell_order: Account<'info, SellOrder>,

    #[account(mut,seeds=[b"market",base_mint.key().as_ref(),quote_mint.key().as_ref()],bump)]
    pub market:Account<'info,Market>,

    #[account(mint::token_program=token_program)]
    pub token_mint:InterfaceAccount<'info,Mint>,

    #[account(mut,associated_token::mint=token_mint,associated_token::authority=user,associated_token::token_program=token_program)]
    pub user_token_account: InterfaceAccount<'info,TokenAccount>,

    #[account(mut,associated_token::mint=token_mint,associated_token::authority=market,associated_token::token_program=token_program,constraint = market.base_mint == token_mint.key() @ ErrorCode::InvalidQuoteMint)]
    pub base_vault:InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,

    pub associated_token_program:Program<'info,AssociatedToken>,

    pub system_program:Program<'info,System>

}