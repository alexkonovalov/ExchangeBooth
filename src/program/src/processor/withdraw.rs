use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{instruction::transfer, state::Account, ID as TOKEN_PROGRAM_ID};

use crate::error::ExchangeBoothError;

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let admin = next_account_info(accounts_iter)?;
    let vault_a = next_account_info(accounts_iter)?;
    let vault_b = next_account_info(accounts_iter)?;
    let receiver_a = next_account_info(accounts_iter)?;
    let receiver_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let vault_a_content = Account::unpack(&vault_a.data.borrow())?;
    let vault_b_content = Account::unpack(&vault_b.data.borrow())?;

    let receiver_a_content = Account::unpack(&receiver_a.data.borrow())?;
    let receiver_b_content = Account::unpack(&receiver_b.data.borrow())?;

    let (oracle_key, _oracle_bump) = Pubkey::find_program_address(
        &[
            admin.key.as_ref(),
            vault_a_content.mint.as_ref(),
            vault_b_content.mint.as_ref(),
        ],
        program_id,
    );

    let (eb_key, _eb_bump) = Pubkey::find_program_address(&[oracle_key.as_ref()], program_id);

    let (vault_a_key, vault_a_bump) = Pubkey::find_program_address(
        &[eb_key.as_ref(), vault_a_content.mint.as_ref()],
        program_id,
    );

    let (vault_b_key, vault_b_bump) = Pubkey::find_program_address(
        &[eb_key.as_ref(), vault_b_content.mint.as_ref()],
        program_id,
    );

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
    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid account address for System Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault_a_content.mint != receiver_a_content.mint {
        msg!("Mint of receiever A does not match with vault A");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault_b_content.mint != receiver_b_content.mint {
        msg!("Mint of receiever B does not match with vault B");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    invoke_signed(
        &transfer(
            token_program.key,
            vault_a.key,
            receiver_a.key,
            vault_a.key,
            &[vault_a.key],
            vault_a_content.amount,
        )?,
        &[token_program.clone(), vault_a.clone(), receiver_a.clone()],
        &[&[
            eb_key.as_ref(),
            vault_a_content.mint.as_ref(),
            &[vault_a_bump],
        ]],
    )?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault_b.key,
            receiver_b.key,
            vault_b.key,
            &[vault_b.key],
            vault_b_content.amount,
        )?,
        &[token_program.clone(), vault_b.clone(), receiver_b.clone()],
        &[&[
            eb_key.as_ref(),
            vault_b_content.mint.as_ref(),
            &[vault_b_bump],
        ]],
    )?;

    Ok(())
}
