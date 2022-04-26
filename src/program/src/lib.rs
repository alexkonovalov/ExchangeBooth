

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
use core::panic;
use std::str::FromStr;
use spl_token::{instruction::{ initialize_account, transfer, close_account }, state::Account};


use crate::commands::ProgramInstruction;
pub mod commands;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExchangeBoothAccount {
    pub vault1: Pubkey,
    pub donor_vault: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OracleAccount {
    pub exchange_rate: f64,
}


// Declare and export the program's entrypoint
entrypoint!(process_instruction);

fn convert_to_u64(amount: f64) -> u64 {
    (amount * f64::powf(10.0.into(), 9.into())) as u64
}


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
        Ok(ProgramInstruction::Exchange { amount: deposited_amount  }) => {
            let user_ai = next_account_info(accounts_iter)?;

            let receiver_vault = next_account_info(accounts_iter)?;
            let donor_vault = next_account_info(accounts_iter)?;
        
            let receiver_account = next_account_info(accounts_iter)?;
            let donor_account = next_account_info(accounts_iter)?;

            let oracle_ai = next_account_info(accounts_iter)?;

            let token_program = next_account_info(accounts_iter)?;

            let oracle = OracleAccount::try_from_slice(&oracle_ai.data.borrow())?;
            let rate = oracle.exchange_rate;

            let donor_account_content = Account::unpack(&donor_account.data.borrow())?;
            let receiver_account_content = Account::unpack(&receiver_account.data.borrow())?;

            msg!("donor_account: {:?}", donor_account_content);
            msg!("receiver_account_content: {:?}",receiver_account_content);
            msg!("rate: {:?}", rate);
            msg!("deposited_amount: {:?}", deposited_amount);

            let donor_mint =  donor_account_content.mint;
            let receivier_mint =  receiver_account_content.mint;

            let (oracle_receiver_to_donor_key, _bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), receivier_mint.as_ref(), donor_mint.as_ref()],
                program_id,
            );

            let (oracle_donor_to_receiver_key, _bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), donor_mint.as_ref(), receivier_mint.as_ref()],
                program_id,
            );

            let (donor_vault_key, donor_vault_bump) = Pubkey::find_program_address(
                &[user_ai.key.as_ref(), receivier_mint.as_ref()],
                program_id,
            );

            msg!("________donor_vault_key : {:?}", donor_vault_key);

            let ff = oracle_ai.key;
            let withdrawn_tokens: u64;

            msg!("oracle key: {:?}", oracle_ai.key);
            msg!("oracle_receiver_to_donor_key: {:?}", oracle_receiver_to_donor_key);
            msg!("oracle_donor_to_receiver_key: {:?}",oracle_donor_to_receiver_key);
            msg!("receivier_mint: {:?}",receivier_mint);
            msg!("oracle_donor_to_receiver_key: {:?}",oracle_donor_to_receiver_key);

            if ff == &oracle_receiver_to_donor_key {
                withdrawn_tokens = convert_to_u64(oracle.exchange_rate * deposited_amount);
            } else if ff == &oracle_donor_to_receiver_key {
                withdrawn_tokens = convert_to_u64( deposited_amount / oracle.exchange_rate);
            } else {
                panic!("incorrect oracle key");
            }

            let deposited_tokens = convert_to_u64(deposited_amount);

            invoke(
                &transfer(
                    token_program.key,
                    donor_account.key,
                    receiver_vault.key,
                    user_ai.key,
                    &[user_ai.key],
                    deposited_tokens,
                )?,
                &[token_program.clone(), receiver_vault.clone(), donor_account.clone(), user_ai.clone()],
            )?;

            invoke_signed(
                &transfer(
                    token_program.key,
                    donor_vault.key,
                    receiver_account.key,
                    donor_vault.key,
                    &[donor_vault.key],
                    withdrawn_tokens,
                )?,
                &[token_program.clone(), donor_vault.clone(), receiver_account.clone(), user_ai.clone()],
                &[&[user_ai.key.as_ref(), receivier_mint.as_ref(), &[donor_vault_bump]]],
            )?;
        },
        Ok(ProgramInstruction::Deposit { amount, amount2  }) => {
            let user_ai = next_account_info(accounts_iter)?;
            let vault1 = next_account_info(accounts_iter)?;
            let vault2 = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let source_mint1_ai = next_account_info(accounts_iter)?;
            let source_mint2_ai = next_account_info(accounts_iter)?;
            let amount = convert_to_u64(amount);
            let amount2 = convert_to_u64(amount2);

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
            let donor_vault = next_account_info(accounts_iter)?;
            let mint1 = next_account_info(accounts_iter)?;
            let mint2 = next_account_info(accounts_iter)?;
            let destination_mint1_ai = next_account_info(accounts_iter)?;
            let destination_mint2_ai = next_account_info(accounts_iter)?;
            let oracle = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;

            let vault1_content = Account::unpack(&vault1.data.borrow())?;
            let vault2_content = Account::unpack(&donor_vault.data.borrow())?;
            
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
                    donor_vault.key,
                    destination_mint2_ai.key,
                    donor_vault.key,
                    &[donor_vault.key],
                    vault2_content.amount,
                )?,
                &[donor_vault.clone(), destination_mint2_ai.clone(), user_ai.clone()],
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
                    donor_vault.key,
                    destination_mint2_ai.key,
                    donor_vault.key,
                    &[donor_vault.key]
                )?,
                &[token_program.clone(), donor_vault.clone(), destination_mint2_ai.clone(), user_ai.clone()],
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
            let donor_vault = next_account_info(accounts_iter)?;
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
                    donor_vault.key,
                    Rent::get()?.minimum_balance(165),
                    165,
                    token_program.key,
                ),
                &[user_ai.clone(), system_program.clone(), token_program.clone(), donor_vault.clone()],
                &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump]]],
            )?;

            invoke_signed(
                &initialize_account(
                    token_program.key,
                    donor_vault.key,
                    mint2.key,
                    donor_vault.key,
                )?,
                &[token_program.clone(), donor_vault.clone(), mint2.clone(), user_ai.clone(), rent_program.clone()],
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
            booth.donor_vault = *donor_vault.key;

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
