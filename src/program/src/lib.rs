

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::str::FromStr;

use crate::commands::ProgramInstruction;
pub mod commands;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
    pub data: [u8; 8],
    pub authority: Pubkey,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    let ix  = ProgramInstruction::unpack(instruction_data);

    msg!("instruction::::: {:?}", ix);
    msg!("program id::::: {:?}", program_id);

    msg!("Goodbye World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    let signer_account = next_account_info(accounts_iter);

    msg!("$$$$$$$ greetedPubkey{:?}", account);
    msg!("$$$$$$$ myAccount{:?}", signer_account);
    let greetk2 = next_account_info(accounts_iter);
    msg!("$$$$$$$ greet_key_2{:?}", greetk2);

    match ix {
        Ok(ProgramInstruction::InitializeExchangeBooth {  }) => {
            // msg!("-------init exchange booth");
        },
        _ => {
            msg!("+++++++ NOT init exchange booth");
        }
    }

    // The account must be owned by the program in order to modify its data
    // if account.owner != program_id {
    //     msg!("Greeted account does not have the correct program id");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    // Increment and store the number of times the account has been greeted
    // let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;


    // if greeting_account.counter == 0 {
    //     greeting_account.authority = Pubkey::from_str("DGUWh9zsVv3XmFGZxkTpdaJUQkkXvUUoWHundLsPjxMH").expect("bad authority pubkey");
    // }

    // greeting_account.counter += 1;
   //  greeting_account.data = instruction_data;

    let mut fuf: [u8; 8] = [0; 8];

    match *instruction_data {
        [b1, b2, b3, b4, b5, b6, b7, b8] => {
            fuf = [b1, b2, b3, b4, b5, b6, b7, b8];
        }
        _ => {

        }
    }

    // greeting_account.data = fuf;

    // msg!("greeting account END:: {:?}", greeting_account);

    // greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
