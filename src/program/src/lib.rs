use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};
mod commands;
mod error;
mod helpers;
mod processor;
mod state;

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
