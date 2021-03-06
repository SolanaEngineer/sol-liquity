import * as anchor from '@project-serum/anchor';
const DEVNET_MODE = true;

export const GLOBAL_STATE_TAG = "golbal-state-seed";
export const TOKEN_VAULT_TAG = "token-vault-seed";
export const USER_TROVE_TAG = "user-trove-seed";
export const SOLUSD_MINT_TAG = "solusd-mint";
export const TOKEN_VAULT_POOL_TAG = "token-vault-pool";
export const STABILITY_POOL_TAG = "stability-pool";
export const SOLUSD_DECIMALS = 6;

export const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
export const SYSVAR_RENT_PUBKEY = new anchor.web3.PublicKey('SysvarRent111111111111111111111111111111111');
export const SYSVAR_CLOCK_PUBKEY = new anchor.web3.PublicKey('SysvarC1ock11111111111111111111111111111111');
export const SYSTEM_PROGRAM_ID = new anchor.web3.PublicKey('11111111111111111111111111111111');

export const LIQUITY_PROGRAM_ID = new anchor.web3.PublicKey('GYiTbo5gmrPfTNaHwBhdW4Dq7taRaNADwhp9iRBfxgQ4');
export const PYTH_PROGRAM_ID = new anchor.web3.PublicKey(DEVNET_MODE?'gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s':'FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH');
export const PYTH_PRODUCT_SOL = new anchor.web3.PublicKey(DEVNET_MODE?'3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E':'ALP8SdU9oARYVLgLR7LrqMNCYBnhtnQz1cj6bwgwQmgj');
export const PYTH_PRICE_SOL = new anchor.web3.PublicKey(DEVNET_MODE?'J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix':'H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG');

export const SOL_MINT_ADDRESS = new anchor.web3.PublicKey('So11111111111111111111111111111111111111112');
