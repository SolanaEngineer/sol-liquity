//! All instruction types
//! These instructions represent a function what will be processed by this program

// this allows many arguments for the function parameters
#![allow(clippy::too_many_arguments)]

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        // sysvar
    },
};

/// Instructions supported by the Borrower Operations program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum BorrowerOperationsInstruction {
    ///   Initializes a new Borrower Operations.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` New Borrower Operations account to create.
    ///   1. `[]` authority to initialize this pool account
    ///   4. `[]` nonce
    ///   5. `[]` program id
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },

    /// Provide to Borrower Operations
    ///
    /// If caller has a pre-existing stake, send any accumulated SOL and solUSD gains to them. 
    /// 
    ///   0. `[w]` SOLIDStaking account to stake
    ///   1. `[]` authority of this pool account
    ///   7. `[]` Token program id
    ///   8. `[]` program id
    ///   9. `[]` amount
    OpenTrove{
        max_fee_percentage: u64,

        solusd_amount: u64,
        
        coll_increase:u64,
    },

    AdjustTrove{
        coll_withdrawal: u64,
    
        solusd_change: u64,
        
        is_debt_increase:u8,
        
        max_fee_percentage: u64,
        
        sol_amount: u64
    },

    CloseTrove(u64),
}