use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive, PartialEq)]
pub enum ExchangeBoothError {
    #[error("Missing required signature.")]
    MissingRequiredSignature,
    #[error("Invalid account address.")]
    InvalidAccountAddress,
    #[error("Compute error.")]
    ComputeError,
    #[error("Fee exceeds 100%")]
    FeeOverMaxError,
    #[error("Conversion results in zero token amount.")]
    TooSmallAmountError,
    #[error("Conversion error.")]
    ConversionError,
}

impl From<ExchangeBoothError> for ProgramError {
    fn from(e: ExchangeBoothError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
