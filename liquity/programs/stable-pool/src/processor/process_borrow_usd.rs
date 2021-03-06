use anchor_lang::prelude::*;
use anchor_spl::token::{self,  MintTo};

use crate::{
    constant::*,
    instructions::*,
    utils::*,
};

pub fn process_borrow_usd(ctx: Context<BorrowUsd>, amount: u64, _token_vault_nonce: u8, _user_trove_nonce: u8, _global_state_nonce: u8, _mint_usd_nonce: u8) -> ProgramResult {
    let market_price = get_market_price(
        *ctx.accounts.oracle_program.key,
        &ctx.accounts.pyth_product,
        &ctx.accounts.pyth_price,
        &ctx.accounts.clock
    )?;

    assert_debt_allowed(ctx.accounts.user_trove.coll, ctx.accounts.user_trove.debt, amount, market_price, ctx.accounts.mint_coll.decimals, ctx.accounts.mint_usd.decimals)?;

    // mint to user
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_usd.to_account_info().clone(),
        to: ctx.accounts.user_token_usd.to_account_info().clone(),
        authority: ctx.accounts.global_state.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();

    let signer_seeds = &[
        GLOBAL_STATE_TAG,
        &[_global_state_nonce],
    ];
    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, amount)?;

    ctx.accounts.token_vault.total_debt += amount;
    ctx.accounts.user_trove.debt += amount;

    Ok(())
}
