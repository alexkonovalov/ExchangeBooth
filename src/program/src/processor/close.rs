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

    let admin_ai = next_account_info(accounts_iter)?;
    let eb_ai = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let mint1 = next_account_info(accounts_iter)?;
    let mint2 = next_account_info(accounts_iter)?;
    let destination_mint1_ai = next_account_info(accounts_iter)?;
    let destination_mint2_ai = next_account_info(accounts_iter)?;
    let oracle_ai = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let (eb_key, _eb_bump) = Pubkey::find_program_address(&[oracle_ai.key.as_ref()], program_id);

    let (vault1_key, vault1_bump) =
        Pubkey::find_program_address(&[eb_ai.key.as_ref(), mint1.key.as_ref()], program_id);

    let (vault2_key, vault2_bump) =
        Pubkey::find_program_address(&[eb_ai.key.as_ref(), mint2.key.as_ref()], program_id);

    let (oracle_key, _oracle_bump) = Pubkey::find_program_address(
        &[
            admin_ai.key.as_ref(),
            mint1.key.as_ref(),
            mint2.key.as_ref(),
        ],
        program_id,
    );

    if !admin_ai.is_signer {
        msg!("No signature for booth admin");
        return Err(ExchangeBoothError::MissingRequiredSignature.into());
    }

    if vault1_key != *vault1.key {
        msg!("Invalid account address for Vault 1");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_key != *vault2.key {
        msg!("Invalid account address for Vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if oracle_key != *oracle_ai.key {
        msg!("Invalid account address for Vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if eb_key != *eb_ai.key {
        msg!("Invalid account address for Vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid account address for System Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    let vault1_content = Account::unpack(&vault1.data.borrow())?;
    let vault2_content = Account::unpack(&vault2.data.borrow())?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault1.key,
            destination_mint1_ai.key,
            vault1.key,
            &[vault1.key],
            vault1_content.amount,
        )?,
        &[vault1.clone(), destination_mint1_ai.clone()],
        &[&[eb_key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault2.key,
            destination_mint2_ai.key,
            vault2.key,
            &[vault2.key],
            vault2_content.amount,
        )?,
        &[vault2.clone(), destination_mint2_ai.clone()],
        &[&[eb_key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]],
    )?;

    invoke_signed(
        &close_account(
            token_program.key,
            vault1.key,
            destination_mint1_ai.key,
            vault1.key,
            &[vault1.key],
        )?,
        &[
            token_program.clone(),
            vault1.clone(),
            destination_mint1_ai.clone(),
        ],
        &[&[eb_ai.key.as_ref(), mint1.key.as_ref(), &[vault1_bump]]],
    )?;

    invoke_signed(
        &close_account(
            token_program.key,
            vault2.key,
            destination_mint2_ai.key,
            vault2.key,
            &[vault2.key],
        )?,
        &[
            token_program.clone(),
            vault2.clone(),
            destination_mint2_ai.clone(),
        ],
        &[&[eb_ai.key.as_ref(), mint2.key.as_ref(), &[vault2_bump]]],
    )?;

    **admin_ai.try_borrow_mut_lamports()? = admin_ai
        .lamports()
        .checked_add(eb_ai.lamports())
        .ok_or(ExchangeBoothError::ComputeError)?
        .checked_add(oracle_ai.lamports())
        .ok_or(ExchangeBoothError::ComputeError)?;
    *eb_ai.try_borrow_mut_data()? = &mut [];
    **eb_ai.try_borrow_mut_lamports()? = 0;
    *oracle_ai.try_borrow_mut_data()? = &mut [];
    **oracle_ai.try_borrow_mut_lamports()? = 0;

    Ok(())
}
