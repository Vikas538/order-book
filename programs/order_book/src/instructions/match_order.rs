use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
token_interface::{Mint,TokenAccount,TokenInterface,transfer_checked,TransferChecked},
};

use crate::state::{BuyOrder, Market, SellOrder};
use crate::state::errors::ErrorCode;

pub fn match_order(ctx: Context<MatchOrder>, _buy_order_id: u64, _sell_order_id: u64) -> Result<()> {
    // 1. Calculate the match amount (The smaller of the two remaining quantities)
    let match_amount = std::cmp::min(
        ctx.accounts.buy_order.remaining,
        ctx.accounts.sell_order.remaining
    );

    if match_amount == 0 {
        return err!(ErrorCode::InvalidOrder);
    }

    // 2. Calculate USDC to transfer (Base Quantity * Price / 1e9)
    let usdc_to_transfer = match_amount
        .checked_mul(ctx.accounts.buy_order.buy_price).unwrap()
        .checked_div(1_000_000_000).unwrap();

    let seeds = &[
        b"market",
        ctx.accounts.market.base_mint.as_ref(),
        ctx.accounts.market.quote_mint.as_ref(),
        &[ctx.accounts.market.bump],
    ];
    let signer = &[&seeds[..]];

    // --- STEP 1: Transfer USDC from Quote Vault to Seller ---
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.quote_vault.to_account_info(), // From Vault
                to: ctx.accounts.seller_quote_ata.to_account_info(), // To Seller
                mint: ctx.accounts.quote_mint.to_account_info(),
                authority: ctx.accounts.market.to_account_info(),
            },
            signer,
        ),
        usdc_to_transfer,
        ctx.accounts.quote_mint.decimals,
    )?;

    // --- STEP 2: Transfer SOL from Base Vault to Buyer ---
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.base_vault.to_account_info(), // From Vault
                to: ctx.accounts.buyer_base_ata.to_account_info(), // To Buyer
                mint: ctx.accounts.base_mint.to_account_info(),
                authority: ctx.accounts.market.to_account_info(),
            },
            signer,
        ),
        match_amount, // The SOL quantity
        ctx.accounts.base_mint.decimals,
    )?;

    // --- STEP 3: Update State ---
    ctx.accounts.buy_order.remaining -= match_amount;
    ctx.accounts.buy_order.quantity_filled += match_amount;
    
    ctx.accounts.sell_order.remaining -= match_amount;
    ctx.accounts.sell_order.quantity_filled += match_amount;

    if ctx.accounts.buy_order.remaining == 0 {
        ctx.accounts.buy_order.is_filled = true;
    }
    if ctx.accounts.sell_order.remaining == 0 {
        ctx.accounts.sell_order.is_filled = true;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(base_mint:Pubkey,quote_mint:Pubkey,buy_order_id:u64,sell_order_id:u64)]
pub struct MatchOrder<'info>{
    #[account(mut,seeds=[b"market",base_mint.key().as_ref(),quote_mint.key().as_ref()],bump)]
    pub market:Account<'info,Market>,

    #[account(mut,seeds=[b"buy_limit_order",buy_order_id.to_le_bytes().as_ref()],bump=buy_order.bump)]
    pub buy_order:Account<'info,BuyOrder>,

    #[account(mut,seeds=[b"sell_limit_order",sell_order_id.to_le_bytes().as_ref()],bump=sell_order.bump)]
    pub sell_order:Account<'info,SellOrder>,

    #[account(mint::token_program=token_program)]
    pub base_mint:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program)]
    pub quote_mint:InterfaceAccount<'info,Mint>,

    #[account(mut,associated_token::mint=base_mint,associated_token::authority=market,associated_token::token_program=token_program)]
    pub base_vault:InterfaceAccount<'info,TokenAccount>,

    #[account(mut,associated_token::mint=quote_mint,associated_token::authority=market,associated_token::token_program=token_program)]
    pub quote_vault:InterfaceAccount<'info,TokenAccount>,


    // buyers token account fro receiving wsol
    #[account(mut,associated_token::mint=base_mint,associated_token::authority=buy_order.owner,associated_token::token_program=token_program)]
    pub buyer_base_ata: InterfaceAccount<'info,TokenAccount>,

    // sellers token account for receiving usdc
    #[account(mut,associated_token::mint=quote_mint,associated_token::authority=sell_order.owner,associated_token::token_program=token_program)]
    pub seller_quote_ata: InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,

    pub associated_token_program:Program<'info,AssociatedToken>,

    pub system_program:Program<'info,System>,
}