use crate::{
    error::ExchangeBoothError,
    state::{ExchangeBoothAccount, OracleAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction::{self},
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::{instruction::initialize_account, ID as TOKEN_PROGRAM_ID};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], exchange_rate: f64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

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

    //todo user shouldnt be in seeds for token
    let (vault1_key, vault1_bump) =
        Pubkey::find_program_address(&[user_ai.key.as_ref(), mint1.key.as_ref()], program_id);

    //todo user shouldnt be in seeds for token
    let (vault2_key, vault2_bump) =
        Pubkey::find_program_address(&[user_ai.key.as_ref(), mint2.key.as_ref()], program_id);

    let (oracle_key, oracle_bump) = Pubkey::find_program_address(
        &[user_ai.key.as_ref(), mint1.key.as_ref(), mint2.key.as_ref()],
        program_id,
    );

    let (eb_key, eb_bump) = Pubkey::find_program_address(&[oracle_ai.key.as_ref()], program_id);

    if vault1_key != *vault1.key {
        msg!("Invalid account address for Vault 1");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_key != *vault2.key {
        msg!("Invalid account address for Vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if oracle_key != *oracle_ai.key {
        msg!("Invalid account address for Oracle");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if eb_key != *eb_ai.key {
        msg!("Invalid account address for Exchange Booth");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if SYSTEM_PROGRAM_ID != *system_program.key {
        msg!("Invalid account address for System Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid account address for Token Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if sysvar::rent::id() != *rent_program.key {
        msg!("Invalid account address for Rent Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    invoke_signed(
        &system_instruction::create_account(
            user_ai.key,
            vault1.key,
            Rent::get()?.minimum_balance(165),
            165,
            token_program.key,
        ),
        &[user_ai.clone(), system_program.clone(), vault1.clone()],
        &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &initialize_account(token_program.key, vault1.key, mint1.key, vault1.key)?,
        &[
            token_program.clone(),
            vault1.clone(),
            mint1.clone(),
            rent_program.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            user_ai.key,
            vault2.key,
            Rent::get()?.minimum_balance(165),
            165,
            token_program.key,
        ),
        &[user_ai.clone(), system_program.clone(), vault2.clone()],
        &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]],
    )?;

    invoke_signed(
        &initialize_account(token_program.key, vault2.key, mint2.key, vault2.key)?,
        &[
            token_program.clone(),
            vault2.clone(),
            mint2.clone(),
            rent_program.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            user_ai.key,
            oracle_ai.key,
            Rent::get()?.minimum_balance(8),
            8,
            program_id,
        ),
        &[user_ai.clone(), oracle_ai.clone(), system_program.clone()],
        &[&[
            user_ai.key.as_ref(),
            mint1.key.as_ref(),
            mint2.key.as_ref(),
            &[oracle_bump],
        ]],
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

    Ok(())
}
