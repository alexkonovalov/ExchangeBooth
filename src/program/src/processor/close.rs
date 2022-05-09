use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{close_account, transfer},
    state::Account,
    ID as TOKEN_PROGRAM_ID,
};

use crate::error::ExchangeBoothError;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let admin = next_account_info(accounts_iter)?;
    let eb = next_account_info(accounts_iter)?;
    let vault_a = next_account_info(accounts_iter)?;
    let vault_b = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let destination_a = next_account_info(accounts_iter)?;
    let destination_b = next_account_info(accounts_iter)?;
    let oracle = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let (eb_key, _eb_bump) = Pubkey::find_program_address(&[oracle.key.as_ref()], program_id);

    let (vault1_key, vault1_bump) =
        Pubkey::find_program_address(&[eb.key.as_ref(), mint_a.key.as_ref()], program_id);

    let (vault2_key, vault2_bump) =
        Pubkey::find_program_address(&[eb.key.as_ref(), mint_b.key.as_ref()], program_id);

    let (oracle_key, _oracle_bump) = Pubkey::find_program_address(
        &[admin.key.as_ref(), mint_a.key.as_ref(), mint_b.key.as_ref()],
        program_id,
    );

    if !admin.is_signer {
        msg!("No signature for booth admin");
        return Err(ExchangeBoothError::MissingRequiredSignature.into());
    }

    if vault1_key != *vault_a.key {
        msg!("Invalid account address for Vault A");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_key != *vault_b.key {
        msg!("Invalid account address for Vault B");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if oracle_key != *oracle.key {
        msg!("Invalid account address for Oracle");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if eb_key != *eb.key {
        msg!("Invalid account address for Exchange Booth");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid account address for Token Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    let vault1_content = Account::unpack(&vault_a.data.borrow())?;
    let vault2_content = Account::unpack(&vault_b.data.borrow())?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault_a.key,
            destination_a.key,
            vault_a.key,
            &[vault_a.key],
            vault1_content.amount,
        )?,
        &[vault_a.clone(), destination_a.clone()],
        &[&[eb_key.as_ref(), mint_a.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault_b.key,
            destination_b.key,
            vault_b.key,
            &[vault_b.key],
            vault2_content.amount,
        )?,
        &[vault_b.clone(), destination_b.clone()],
        &[&[eb_key.as_ref(), mint_b.key.as_ref(), &[vault2_bump]]],
    )?;

    invoke_signed(
        &close_account(
            token_program.key,
            vault_a.key,
            destination_a.key,
            vault_a.key,
            &[vault_a.key],
        )?,
        &[
            token_program.clone(),
            vault_a.clone(),
            destination_a.clone(),
        ],
        &[&[eb.key.as_ref(), mint_a.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &close_account(
            token_program.key,
            vault_b.key,
            destination_b.key,
            vault_b.key,
            &[vault_b.key],
        )?,
        &[
            token_program.clone(),
            vault_b.clone(),
            destination_b.clone(),
        ],
        &[&[eb.key.as_ref(), mint_b.key.as_ref(), &[vault2_bump]]],
    )?;

    **admin.try_borrow_mut_lamports()? = admin
        .lamports()
        .checked_add(eb.lamports())
        .ok_or(ExchangeBoothError::ComputeError)?
        .checked_add(oracle.lamports())
        .ok_or(ExchangeBoothError::ComputeError)?;
    *eb.try_borrow_mut_data()? = &mut [];
    **eb.try_borrow_mut_lamports()? = 0;
    *oracle.try_borrow_mut_data()? = &mut [];
    **oracle.try_borrow_mut_lamports()? = 0;

    Ok(())
}
