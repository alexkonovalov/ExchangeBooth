use crate::commands::ProgramInstruction;
use crate::processor;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

mod close;
mod deposit;
mod exchange;
mod initialize;
mod withdraw;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = ProgramInstruction::unpack(instruction_data);
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

    Ok(())
}
