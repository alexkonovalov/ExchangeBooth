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

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    exchange_rate: u64,
    rate_decimals: u8,
    fee: u64,
    fee_decimals: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let admin = next_account_info(accounts_iter)?;
    let eb = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let vault_a = next_account_info(accounts_iter)?;
    let vault_b = next_account_info(accounts_iter)?;
    let oracle_ai = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_program = next_account_info(accounts_iter)?;

    let (oracle_key, oracle_bump) = Pubkey::find_program_address(
        &[admin.key.as_ref(), mint_a.key.as_ref(), mint_b.key.as_ref()],
        program_id,
    );

    // so far oracle and exchange booth are 1 to 1 connected
    let (eb_key, eb_bump) = Pubkey::find_program_address(&[oracle_ai.key.as_ref()], program_id);

    let (vault_a_key, vault_a_bump) =
        Pubkey::find_program_address(&[eb.key.as_ref(), mint_a.key.as_ref()], program_id);

    let (vault_b_key, vault_b_bump) =
        Pubkey::find_program_address(&[eb.key.as_ref(), mint_b.key.as_ref()], program_id);

    if !admin.is_signer {
        msg!("No signature for booth admin");
        return Err(ExchangeBoothError::MissingRequiredSignature.into());
    }

    if vault_a_key != *vault_a.key {
        msg!("Invalid account address for Vault A");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault_b_key != *vault_b.key {
        msg!("Invalid account address for Vault B");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if oracle_key != *oracle_ai.key {
        msg!("Invalid account address for Oracle");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if eb_key != *eb.key {
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
            admin.key,
            vault_a.key,
            Rent::get()?.minimum_balance(165),
            165,
            token_program.key,
        ),
        &[admin.clone(), system_program.clone(), vault_a.clone()],
        &[&[eb.key.as_ref(), mint_a.key.as_ref(), &[vault_a_bump]]],
    )?;

    invoke_signed(
        &initialize_account(token_program.key, vault_a.key, mint_a.key, vault_a.key)?,
        &[
            token_program.clone(),
            vault_a.clone(),
            mint_a.clone(),
            rent_program.clone(),
        ],
        &[&[eb.key.as_ref(), mint_a.key.as_ref(), &[vault_a_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            vault_b.key,
            Rent::get()?.minimum_balance(165),
            165,
            token_program.key,
        ),
        &[admin.clone(), system_program.clone(), vault_b.clone()],
        &[&[eb.key.as_ref(), mint_b.key.as_ref(), &[vault_b_bump]]],
    )?;

    invoke_signed(
        &initialize_account(token_program.key, vault_b.key, mint_b.key, vault_b.key)?,
        &[
            token_program.clone(),
            vault_b.clone(),
            mint_b.clone(),
            rent_program.clone(),
        ],
        &[&[eb.key.as_ref(), mint_b.key.as_ref(), &[vault_b_bump]]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            oracle_ai.key,
            Rent::get()?.minimum_balance(9),
            9,
            program_id,
        ),
        &[admin.clone(), oracle_ai.clone(), system_program.clone()],
        &[&[
            admin.key.as_ref(),
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[oracle_bump],
        ]],
    )?;

    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            eb.key,
            Rent::get()?.minimum_balance(9),
            9,
            program_id,
        ),
        &[admin.clone(), eb.clone(), system_program.clone()],
        &[&[oracle_ai.key.as_ref(), &[eb_bump]]],
    )?;

    let mut booth = ExchangeBoothAccount::try_from_slice(&eb.data.borrow())?;
    booth.fee = fee;
    booth.decimals = fee_decimals;

    booth.serialize(&mut *eb.data.borrow_mut())?;

    let mut oracle = OracleAccount::try_from_slice(&oracle_ai.data.borrow())?;
    oracle.exchange_rate = exchange_rate;
    oracle.decimals = rate_decimals;

    oracle.serialize(&mut *oracle_ai.data.borrow_mut())?;

    Ok(())
}
