

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke_signed, invoke},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey, system_instruction::{self, SystemError },
    sysvar::{rent::Rent, Sysvar}, program_pack::Pack,
};
use std::str::FromStr;
use spl_token::{instruction::{ initialize_account, transfer, close_account }, state::Account};


use crate::commands::ProgramInstruction;
pub mod commands;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExchangeBoothAccount {
    pub vault1: Pubkey,
    pub vault2: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OracleAccount {
    pub exchange_rate: f64,
}


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
    // msg!("program id::::: {:?}", program_id);
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();
    // Get the account to say hello to

    match ix {
        Ok(ProgramInstruction::Deposit { amount, amount2  }) => {
            let user_ai = next_account_info(accounts_iter)?;
            let vault1 = next_account_info(accounts_iter)?;
            let vault2 = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let source_mint1_ai = next_account_info(accounts_iter)?;
            let source_mint2_ai = next_account_info(accounts_iter)?;
            let amount = (amount * f64::powf(10.0.into(), 9.into())) as u64;
            let amount2 = (amount2 * f64::powf(10.0.into(), 9.into())) as u64;

            invoke(
                &transfer(
                    token_program.key,
                    source_mint1_ai.key,
                    vault1.key,
                    user_ai.key,
                    &[user_ai.key],
                    amount,
                )?,
                &[token_program.clone(), vault1.clone(), source_mint1_ai.clone(), user_ai.clone()],
            )?;

            invoke(
                &transfer(
                    token_program.key,
                    source_mint2_ai.key,
                    vault2.key,
                    user_ai.key,
                    &[user_ai.key],
                    amount2,
                )?,
                &[token_program.clone(), vault2.clone(), source_mint2_ai.clone(), user_ai.clone()],
            )?;

        },
        Ok(ProgramInstruction::CloseExchangeBooth {  }) => {
            let user_ai = next_account_info(accounts_iter)?;
            let eb_ai = next_account_info(accounts_iter)?;
            let vault1 = next_account_info(accounts_iter)?;
            let vault2 = next_account_info(accounts_iter)?;
            let mint1 = next_account_info(accounts_iter)?;
            let mint2 = next_account_info(accounts_iter)?;
            let destination_mint1_ai = next_account_info(accounts_iter)?;
            let destination_mint2_ai = next_account_info(accounts_iter)?;
            let oracle = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;

            let vault1_content = Account::unpack(&vault1.data.borrow())?;
            let vault2_content = Account::unpack(&vault2.data.borrow())?;
            
            let (_vault1_key, bump1) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint1.key.as_ref()],
                program_id,
            );

            invoke_signed(
                &transfer(
                    token_program.key,
                    vault1.key,
                    destination_mint1_ai.key,
                    vault1.key,
                    &[vault1.key],
                    vault1_content.amount,
                )?,
                &[vault1.clone(), destination_mint1_ai.clone(), user_ai.clone()],
                &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump1]]],
            )?;

            let (_vault2_key, bump2) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint2.key.as_ref()],
                program_id,
            );

            invoke_signed(
                &transfer(
                    token_program.key,
                    vault2.key,
                    destination_mint2_ai.key,
                    vault2.key,
                    &[vault2.key],
                    vault2_content.amount,
                )?,
                &[vault2.clone(), destination_mint2_ai.clone(), user_ai.clone()],
                &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump2]]],
            )?;

            invoke_signed(
                &close_account(
                    token_program.key,
                    vault1.key,
                    destination_mint1_ai.key,
                    vault1.key,
                    &[vault1.key]
                )?,
                &[token_program.clone(), vault1.clone(), destination_mint1_ai.clone(), user_ai.clone()],
                &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump1]]],
            )?;

            invoke_signed(
                &close_account(
                    token_program.key,
                    vault2.key,
                    destination_mint2_ai.key,
                    vault2.key,
                    &[vault2.key]
                )?,
                &[token_program.clone(), vault2.clone(), destination_mint2_ai.clone(), user_ai.clone()],
                &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump2]]],
            )?;

             **user_ai.try_borrow_mut_lamports()? = user_ai
                    .lamports()
                    .checked_add(eb_ai.lamports())
                    .ok_or(ProgramError::InsufficientFunds)?//todo find better error
                    .checked_add(oracle.lamports())
                    .ok_or(ProgramError::InsufficientFunds)?; //todo find better error
             *eb_ai.try_borrow_mut_data()? = &mut [];
             **eb_ai.try_borrow_mut_lamports()? = 0;
             *oracle.try_borrow_mut_data()? = &mut [];
             **oracle.try_borrow_mut_lamports()? = 0;

        }
        Ok(ProgramInstruction::InitializeExchangeBooth { exchange_rate  }) => {
            let user_ai = next_account_info(accounts_iter)?;
            let eb_ai = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;
            let mint1 = next_account_info(accounts_iter)?;
            let mint2 = next_account_info(accounts_iter)?;
            let vault1 = next_account_info(accounts_iter)?;
            let vault2 = next_account_info(accounts_iter)?;
            let oracle_ai = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let rent_program = next_account_info(accounts_iter)?;
            let (_vault1_key, bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint1.key.as_ref()],
                program_id,
            );

            
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

            let (_vault2_key, bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint2.key.as_ref()],
                program_id,
            );

            invoke_signed(
                &system_instruction::create_account(
                       user_ai.key,
                    vault2.key,
                    Rent::get()?.minimum_balance(165),
                    165,
                    token_program.key,
                ),
                &[user_ai.clone(), system_program.clone(), token_program.clone(), vault2.clone()],
                &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump]]],
            )?;

            invoke_signed(
                &initialize_account(
                    token_program.key,
                    vault2.key,
                    mint2.key,
                    vault2.key,
                )?,
                &[token_program.clone(), vault2.clone(), mint2.clone(), user_ai.clone(), rent_program.clone()],
                &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump]]],
            )?;

  

            let (_oracle_key, oracle_bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), mint1.key.as_ref(), mint2.key.as_ref()],
                program_id,
            );

            let (_eb_key, eb_bump) = Pubkey::find_program_address(
                &[oracle_ai.key.as_ref()],
                program_id,
            );
                        
            msg!("--------oracle key {:?}", oracle_ai.key);
            msg!("--------oracle bump {:?}", oracle_bump);
            msg!("--------eb key {:?}", eb_ai.key);
            msg!("--------eb bump {:?}",eb_bump);
            msg!("-------_exchange rate {:?}", exchange_rate);
 
            invoke_signed(
                &system_instruction::create_account(
                    user_ai.key,
                    oracle_ai.key,
                    Rent::get()?.minimum_balance(64),
                    8,
                    program_id,
                ),
                &[user_ai.clone(), oracle_ai.clone(), system_program.clone()],
                &[&[user_ai.key.as_ref(), mint1.key.as_ref(), mint2.key.as_ref(), &[oracle_bump]]],
            )?;

            invoke_signed(
                &system_instruction::create_account(
                    user_ai.key,
                    eb_ai.key,
                    Rent::get()?.minimum_balance(64),
                    64,
                    program_id,
                ),
                &[user_ai.clone(), eb_ai.clone(), system_program.clone()],
                &[&[oracle_ai.key.as_ref(), &[eb_bump]]],
            )?;

            let mut booth = ExchangeBoothAccount::try_from_slice(&eb_ai.data.borrow())?;
            booth.vault1 = *vault1.key;
            booth.vault2 = *vault2.key;

            booth.serialize(&mut *eb_ai.data.borrow_mut())?;

            let mut oracle = OracleAccount::try_from_slice(&oracle_ai.data.borrow())?;
            oracle.exchange_rate = exchange_rate;
            oracle.serialize(&mut *oracle_ai.data.borrow_mut())?;
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
