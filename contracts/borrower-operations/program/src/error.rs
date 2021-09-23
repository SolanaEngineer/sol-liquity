//! All error types for this program

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the StabilityPool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum BorrowerOperationsError {
    // 0.
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The borrower operations state is invalid.
    #[error("InvalidState")]
    InvalidState,
    
    /// account's owner is invalid.
    #[error("InvalidOwner")]
    InvalidOwner,

    
    #[error("Max fee percentage must less than or equal to 100%")]
    ExceedMaxFeePercentage,

    #[error("Max fee percentage must be between 0.5% and 100%")]
    InvalidMaxFeePercentage,
    
    #[error("BorrowerOps: Trove is active")]
    TroveIsActive,

    #[error("BorrowerOps: Trove's net debt must be greater than minimum")]
    InvalidNetDebt,

    #[error("BorrowerOps: Trove's composite debt must be greater than zero")]
    InvalidCompositeDebt,
}
impl From<BorrowerOperationsError> for ProgramError {
    fn from(e: BorrowerOperationsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for BorrowerOperationsError {
    fn type_of() -> &'static str {
        "Borrower Operations Error"
    }
} 