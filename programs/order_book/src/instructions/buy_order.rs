use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
token_interface::{Mint,TokenAccount,TokenInterface,transfer_checked,TransferChecked},
};


use crate::state::{BuyOrder,Market};
use crate::state::errors::ErrorCode;

pub fn buy_order(ctx:Context<BuyOrderLimit>,_id: u64,quantity: u64,buy_price: u64,base_mint: Option<Pubkey>,quote_mint: Option<Pubkey>) -> Result<()> {

    ctx.accounts.buy_order.id = _id;
    ctx.accounts.buy_order.owner = ctx.accounts.user.key();
    ctx.accounts.buy_order.quantity = quantity;
    ctx.accounts.buy_order.buy_price = buy_price;
    ctx.accounts.buy_order.is_filled = false;
    ctx.accounts.buy_order.base_mint = base_mint;
    ctx.accounts.buy_order.quote_mint = quote_mint;
    ctx.accounts.buy_order.created_at = Clock::get()?.unix_timestamp as u64;
    ctx.accounts.buy_order.bump = ctx.bumps.buy_order;
    msg!("Buy order created successfully");

    let total_usdc_to_pull = quantity
        .checked_mul(buy_price).unwrap()
        .checked_div(1_000_000_000).unwrap();

    transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked{
        from: ctx.accounts.user_token_account.to_account_info(),
        to:ctx.accounts.quote_vault.to_account_info(),
        authority:ctx.accounts.user.to_account_info(),
        mint:ctx.accounts.token_mint.to_account_info(),
    }),total_usdc_to_pull,ctx.accounts.token_mint.decimals)?;
    Ok(())
}


#[derive(Accounts)]
#[instruction(id: u64,base_mint: Pubkey,quote_mint: Pubkey)]
pub struct BuyOrderLimit<'info> {

    #[account(init,payer = user, space = 8 + BuyOrder::INIT_SPACE,seeds =[b"buy_limit_order",user.key().as_ref(),id.to_le_bytes().as_ref()],bump)]
    pub buy_order: Account<'info, BuyOrder>,

    #[account(mint::token_program=token_program)]
    pub token_mint:InterfaceAccount<'info,Mint>,

    #[account(mut,seeds=[b"market",base_mint.key().as_ref(),quote_mint.key().as_ref()],bump)]
    pub market:Account<'info,Market>,

    #[account(mut,associated_token::mint=token_mint,associated_token::authority=user,associated_token::token_program=token_program)]
    pub user_token_account: InterfaceAccount<'info,TokenAccount>,

    
    #[account(mut,associated_token::mint=token_mint,associated_token::authority=market,associated_token::token_program=token_program,constraint = market.quote_mint == token_mint.key() @ ErrorCode::InvalidQuoteMint)]
    pub quote_vault:InterfaceAccount<'info,TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program:Interface<'info,TokenInterface>,


    pub associated_token_program: Program<'info, AssociatedToken>,
}