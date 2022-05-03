use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction::transfer;

use crate::helpers::convert_to_u64;

pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: f64,
    amount2: f64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

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
        &[
            token_program.clone(),
            vault1.clone(),
            source_mint1_ai.clone(),
            user_ai.clone(),
        ],
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
        &[
            token_program.clone(),
            vault2.clone(),
            source_mint2_ai.clone(),
            user_ai.clone(),
        ],
    )?;

    Ok(())
}
