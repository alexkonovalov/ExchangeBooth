use {borsh::BorshDeserialize, solana_program::program_error::ProgramError};

#[derive(Debug, PartialEq, BorshDeserialize)]
pub enum ProgramInstruction {
    InitializeExchangeBooth {
        exchange_rate: u64,
        rate_decimals: u8,
        fee: u64,
        fee_decimals: u8,
    },
    Deposit {
        amount_a: u64,
        amount_b: u64,
    },
    CloseExchangeBooth {},
    Exchange {
        amount: u64,
    },
    Withdraw {},
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    ToA,
    ToB,
}

impl ProgramInstruction {
    /// Unpack inbound buffer to associated Instruction
    /// The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let payload = ProgramInstruction::try_from_slice(input);

        match payload {
            Ok(ix) => Ok(ix),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}
