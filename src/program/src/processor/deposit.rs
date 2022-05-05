use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{instruction::transfer, state::Account, ID as TOKEN_PROGRAM_ID};

use crate::{error::ExchangeBoothError, helpers::convert_to_u64};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: f64,
    amount2: f64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let admin_ai = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let source_mint1_ai = next_account_info(accounts_iter)?;
    let source_mint2_ai = next_account_info(accounts_iter)?;
    let amount = convert_to_u64(amount);
    let amount2 = convert_to_u64(amount2);

    let vault1_content = Account::unpack(&vault1.data.borrow())?;
    let vault2_content = Account::unpack(&vault2.data.borrow())?;
    let source1_content = Account::unpack(&source_mint1_ai.data.borrow())?;
    let source2_content = Account::unpack(&source_mint2_ai.data.borrow())?;

    let (oracle_key, _oracle_bump) = Pubkey::find_program_address(
        &[
            admin_ai.key.as_ref(),
            vault1_content.mint.as_ref(),
            vault2_content.mint.as_ref(),
        ],
        program_id,
    );

    let (eb_key, _eb_bump) = Pubkey::find_program_address(&[oracle_key.as_ref()], program_id);

    let (vault1_key, _vault1_bump) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault1_content.mint.as_ref()], program_id);

    let (vault2_key, _vault2_bump) =
        Pubkey::find_program_address(&[eb_key.as_ref(), vault2_content.mint.as_ref()], program_id);

    msg!("_______ vault1_key{:?}", vault1_key);
    msg!("_______ vault2_key {:?}", vault2_key);
    msg!("_______ oracle_key {:?}", oracle_key);
    msg!("_______ eb_key {:?}", eb_key);

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
    if vault1_content.mint != source1_content.mint {
        msg!("Mint of source 1 does not match with vault 1");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }
    if vault2_content.mint != source2_content.mint {
        msg!("Mint of source 2 does not match with vault 2");
        return Err(ExchangeBoothError::InvalidAccountAddress.into());
    }

    invoke(
        &transfer(
            token_program.key,
            source_mint1_ai.key,
            vault1.key,
            admin_ai.key,
            &[admin_ai.key],
            amount,
        )?,
        &[
            token_program.clone(),
            vault1.clone(),
            source_mint1_ai.clone(),
            admin_ai.clone(),
        ],
    )?;

    invoke(
        &transfer(
            token_program.key,
            source_mint2_ai.key,
            vault2.key,
            admin_ai.key,
            &[admin_ai.key],
            amount2,
        )?,
        &[
            token_program.clone(),
            vault2.clone(),
            source_mint2_ai.clone(),
            admin_ai.clone(),
        ],
    )?;

    Ok(())
}
