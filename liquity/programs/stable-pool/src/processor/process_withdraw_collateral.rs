use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer, ID};

use crate::{
    states::*,
    error::*,
    constant::*,
    instructions::*
};

pub fn process_withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> ProgramResult {
    let mut _amount = amount;
    if amount > ctx.accounts.user_trove.locked_coll_balance {
        _amount = ctx.accounts.user_trove.locked_coll_balance;
    }
    
    // transfer from pool to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_token_coll.to_account_info().clone(),
        to: ctx.accounts.user_token_coll.to_account_info().clone(),
        authority: ctx.accounts.token_vault.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.clone();
    
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    
    token::transfer(cpi_ctx, _amount)?;

    ctx.accounts.token_vault.total_coll -= amount;
    ctx.accounts.user_trove.locked_coll_balance -= _amount;

    Ok(())
}
