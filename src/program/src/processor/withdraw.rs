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
    let admin_ai = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let receiver1 = next_account_info(accounts_iter)?;
    let receiver2 = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let vault1_content = Account::unpack(&vault1.data.borrow())?;
    let vault2_content = Account::unpack(&vault2.data.borrow())?;

    let receiver1_content = Account::unpack(&receiver1.data.borrow())?;
    let receiver2_content = Account::unpack(&receiver2.data.borrow())?;

    let (oracle_key, _oracle_bump) = Pubkey::find_program_address(
        &[
            admin_ai.key.as_ref(),
            vault1_content.mint.as_ref(),
            vault2_content.mint.as_ref(),
        ],
        program_id,
    );

    let (eb_key, _eb_bump) = Pubkey::find_program_address(&[oracle_key.as_ref()], program_id);

    let (vault1_key, vault1_bump) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault1_content.mint.as_ref()], program_id);

    let (vault2_key, vault2_bump) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault2_content.mint.as_ref()], program_id);

    if vault1_key != *vault1.key {
        msg!("Invalid account address for Vault 1");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_key != *vault2.key {
        msg!("Invalid account address for Vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if TOKEN_PROGRAM_ID != *token_program.key {
        msg!("Invalid account address for System Program");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault1_content.mint != receiver1_content.mint {
        msg!("Mint of receiever 1 does not match with vault 1");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_content.mint != receiver2_content.mint {
        msg!("Mint of receiever 2 does not match with vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    invoke_signed(
        &transfer(
            token_program.key,
            vault1.key,
            receiver1.key,
            vault1.key,
            &[vault1.key],
            vault1_content.amount,
        )?,
        &[token_program.clone(), vault1.clone(), receiver1.clone()],
        &[&[
            eb_key.as_ref(),
            vault1_content.mint.as_ref(),
            &[vault1_bump],
        ]],
    )?;

    invoke_signed(
        &transfer(
            token_program.key,
            vault2.key,
            receiver2.key,
            vault2.key,
            &[vault2.key],
            vault2_content.amount,
        )?,
        &[token_program.clone(), vault2.clone(), receiver2.clone()],
        &[&[
            eb_key.as_ref(),
            vault2_content.mint.as_ref(),
            &[vault2_bump],
        ]],
    )?;

    Ok(())
}
