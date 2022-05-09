use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{instruction::transfer, state::Account, ID as TOKEN_PROGRAM_ID};

use crate::error::ExchangeBoothError;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount_a: u64,
    amount_b: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let admin = next_account_info(accounts_iter)?;
    let vault_a = next_account_info(accounts_iter)?;
    let vault_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let source_a = next_account_info(accounts_iter)?;
    let source_b = next_account_info(accounts_iter)?;

    let vault1_content = Account::unpack(&vault_a.data.borrow())?;
    let vault2_content = Account::unpack(&vault_b.data.borrow())?;
    let source1_content = Account::unpack(&source_a.data.borrow())?;
    let source2_content = Account::unpack(&source_b.data.borrow())?;

    let (oracle_key, _) = Pubkey::find_program_address(
        &[
            admin.key.as_ref(),
            vault1_content.mint.as_ref(),
            vault2_content.mint.as_ref(),
        ],
        program_id,
    );

    let (eb_key, _) = Pubkey::find_program_address(&[oracle_key.as_ref()], program_id);

    let (vault_a_key, _) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault1_content.mint.as_ref()], program_id);

    let (vault_b_key, _) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault2_content.mint.as_ref()], program_id);

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
    if vault1_content.mint != source1_content.mint {
        msg!("Mint of source A does not match with vault A");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_content.mint != source2_content.mint {
        msg!("Mint of source B does not match with vault B");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    invoke(
        &transfer(
            token_program.key,
            source_a.key,
            vault_a.key,
            admin.key,
            &[admin.key],
            amount_a,
        )?,
        &[
            token_program.clone(),
            vault_a.clone(),
            source_a.clone(),
            admin.clone(),
        ],
    )?;

    invoke(
        &transfer(
            token_program.key,
            source_b.key,
            vault_b.key,
            admin.key,
            &[admin.key],
            amount_b,
        )?,
        &[
            token_program.clone(),
            vault_b.clone(),
            source_b.clone(),
            admin.clone(),
        ],
    )?;

    Ok(())
}
