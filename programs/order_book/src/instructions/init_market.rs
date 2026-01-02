use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken,token_interface::{Mint,TokenAccount,TokenInterface}};
use crate::state::Market;

pub fn initilize_market(ctx:Context<InitializeMarket>,base_mint:Pubkey,quote_mint:Pubkey)->Result<()> {
    ctx.accounts.market.base_mint = base_mint;
    ctx.accounts.market.quote_mint = quote_mint;
    ctx.accounts.market.bump = ctx.bumps.market;
    msg!("Market initialized successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {

    #[account(mut)]
    pub payer:Signer<'info>,

    #[account(init,payer=payer,space=Market::INIT_SPACE,seeds=[b"market",base_mint.key().as_ref(),quote_mint.key().as_ref()],bump)]
    pub market:Account<'info,Market>,

    #[account(mint::token_program=token_program)]
    pub base_mint:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program)]
    pub quote_mint:InterfaceAccount<'info,Mint>,


    #[account(init,payer=payer,associated_token::mint=base_mint,associated_token::authority=market,associated_token::token_program=token_program)]
    pub base_vault:InterfaceAccount<'info,TokenAccount>,

    #[account(init,payer=payer,associated_token::mint=quote_mint,associated_token::authority=market,associated_token::token_program=token_program)]
    pub quote_vault:InterfaceAccount<'info,TokenAccount>,

    pub token_program:Interface<'info,TokenInterface>,

    pub associated_token_program:Program<'info,AssociatedToken>,

    pub system_program:Program<'info,System>,
}


