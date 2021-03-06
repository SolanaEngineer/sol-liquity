use anchor_lang::prelude::*;

use crate::{
    instructions::*
};

pub fn process_create_token_vault(ctx: Context<CreateTokenVault>, _token_vault_nonce:u8, _global_state_nonce:u8, _token_coll_nonce:u8) -> ProgramResult {
    ctx.accounts.token_vault.mint_coll = ctx.accounts.mint_coll.key();
    ctx.accounts.token_vault.token_coll = ctx.accounts.token_coll.key();
    ctx.accounts.token_vault.total_coll = 0;
    ctx.accounts.token_vault.total_debt = 0;

    ctx.accounts.token_vault.oracle_program = *ctx.accounts.oracle_program.key;
    ctx.accounts.token_vault.pyth_product = *ctx.accounts.pyth_product.key;
    ctx.accounts.token_vault.pyth_price = *ctx.accounts.pyth_price.key;
    Ok(())
}
