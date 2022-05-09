use crate::commands::ProgramInstruction;
use crate::processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, log::sol_log_compute_units,
    msg, pubkey::Pubkey,
};

mod close;
mod deposit;
mod exchange;
mod initialize;
mod withdraw;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("instruction_data::::: {:?}", instruction_data);

    let ix = ProgramInstruction::unpack(instruction_data);

    msg!("instruction::::: {:?}", ix);

    match ix {
        Ok(ProgramInstruction::Exchange {
            amount: deposited_amount,
        }) => processor::exchange::process(program_id, accounts, deposited_amount)?,
        Ok(ProgramInstruction::Withdraw {}) => processor::withdraw::process(program_id, accounts)?,
        Ok(ProgramInstruction::Deposit { amount_a, amount_b }) => {
            processor::deposit::process(program_id, accounts, amount_a, amount_b)?
        }
        Ok(ProgramInstruction::CloseExchangeBooth {}) => {
            processor::close::process(program_id, accounts)?
        }
        Ok(ProgramInstruction::InitializeExchangeBooth {
            exchange_rate,
            rate_decimals: decimals,
            fee,
            fee_decimals,
        }) => processor::initialize::process(
            program_id,
            accounts,
            exchange_rate,
            decimals,
            fee,
            fee_decimals,
        )?,
        _ => {}
    }

    msg!("COMPUTE UNITS: {:?}", sol_log_compute_units());

    Ok(())
}
