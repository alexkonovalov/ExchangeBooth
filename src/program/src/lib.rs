

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke_signed, invoke},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey, system_instruction::{self},
    sysvar::{rent::Rent, Sysvar},
};
use std::str::FromStr;
use spl_token::instruction::{ initialize_account, transfer };


use crate::commands::ProgramInstruction;
pub mod commands;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExchangeBoothAccount {
    pub token1: Pubkey,
    pub token2: Pubkey,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    let ix = ProgramInstruction::unpack(instruction_data);

    // msg!("instruction::::: {:?}", ix);
    // msg!("program id::::: {:?}", program_id);

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
  

    match ix {
        Ok(ProgramInstruction::Exchange {  }) => {
            let user_ai = next_account_info(accounts_iter)?;
            let vault1 = next_account_info(accounts_iter)?;
            let vault2 = next_account_info(accounts_iter)?;
            let mint1 = next_account_info(accounts_iter)?;
            let mint2 = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let rent_program = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let token_ai = next_account_info(accounts_iter)?;

            let (_vault1_key, bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint1.key.as_ref()],
                program_id,
            );

            msg!("token info::: {:?}", token_ai);
            msg!("vault info::: {:?}", vault1);
            msg!("user_ai info::: {:?}", user_ai);

            invoke_signed(
                &system_instruction::create_account(
                    user_ai.key,
                    vault1.key,
                    Rent::get()?.minimum_balance(165),
                    165,
                    token_program.key,
                ),
                &[user_ai.clone(), system_program.clone(), token_program.clone(), vault1.clone()],
                &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump]]],
            )?;

            invoke_signed(
                &initialize_account(
                    token_program.key,
                    vault1.key,
                    mint1.key,
                    vault1.key,
                )?,
                &[token_program.clone(), vault1.clone(), mint1.clone(), user_ai.clone(), rent_program.clone()],
                &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump]]],
            )?;

            invoke(
                &transfer(
                    token_program.key,
                    token_ai.key,
                    vault1.key,
                    user_ai.key,
                    &[user_ai.key],
                    100,
                )?,
                &[token_program.clone(), vault1.clone(), token_ai.clone(), user_ai.clone()],
            )?;

            msg!("vault info::: {:?}", vault1);

        },
        Ok(ProgramInstruction::InitializeExchangeBooth {  }) => {
            msg!("-------init exchange booth START");

            let user_ai = next_account_info(accounts_iter)?;
            let eb_ai = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let token1_ai = next_account_info(accounts_iter)?;
            let token2_ai = next_account_info(accounts_iter)?;

            let (eb_key, bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref()/*, &[xtra_seed] */],
                program_id,
            );

            msg!("eb_key {:?}", eb_key);
            msg!("eb_ai.key {:?}", eb_ai.key);
            msg!("token1_ai.key {:?}", token1_ai.key);
            msg!("token2_ai.key {:?}", token2_ai.key);
            msg!("token1_ai {:?}", token1_ai);
            msg!("token2_ai {:?}", token2_ai);

            invoke_signed(
                &system_instruction::create_account(
                    user_ai.key,
                    eb_ai.key,
                    Rent::get()?.minimum_balance(64),
                    64,
                    program_id,
                ),
                &[user_ai.clone(), eb_ai.clone(), system_program.clone()],
                &[&[user_ai.key.as_ref(), &[bump], /*&[xtra_seed]*/]],
            )?;

            let mut booth = ExchangeBoothAccount::try_from_slice(&eb_ai.data.borrow())?;
            booth.token1 = *token1_ai.key;
            booth.token2 = *token2_ai.key;

            booth.serialize(&mut *eb_ai.data.borrow_mut())?;
            
            msg!("-------init exchange booth END");
        }
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
