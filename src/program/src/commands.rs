
use {
    borsh::BorshDeserialize, solana_program::program_error::ProgramError,
};

#[derive(Debug, PartialEq, BorshDeserialize)]
pub enum ProgramInstruction {
    InitializeExchangeBooth {
        exchange_rate: f64,
    },
    Deposit {
        amount: f64,
        amount2: f64,
    },
    CloseExchangeBooth {},
    Exchange {
        amount: f64,
    },
    Withdraw {}
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    /// The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let payload = ProgramInstruction::try_from_slice(input);

        match payload {
            Ok(ix) => Ok(ix),
            _ => Err(ProgramError::InvalidArgument)
        }
    }
}