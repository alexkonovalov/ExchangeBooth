use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{close_account, transfer},
    state::Account,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

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

    let (_vault1_key, bump1) =
        Pubkey::find_program_address(&[user_ai.key.as_ref(), mint1.key.as_ref()], program_id);

    invoke_signed(
        &transfer(
            token_program.key,
            vault1.key,
            destination_mint1_ai.key,
            vault1.key,
            &[vault1.key],
            vault1_content.amount,
        )?,
        &[
            vault1.clone(),
            destination_mint1_ai.clone(),
            user_ai.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump1]]],
    )?;

    let (_vault2_key, bump2) =
        Pubkey::find_program_address(&[user_ai.key.as_ref(), mint2.key.as_ref()], program_id);

    invoke_signed(
        &transfer(
            token_program.key,
            donor_vault.key,
            destination_mint2_ai.key,
            donor_vault.key,
            &[donor_vault.key],
            vault2_content.amount,
        )?,
        &[
            donor_vault.clone(),
            destination_mint2_ai.clone(),
            user_ai.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump2]]],
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
            user_ai.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint1.key.as_ref(), &[bump1]]],
    )?;

    invoke_signed(
        &close_account(
            token_program.key,
            donor_vault.key,
            destination_mint2_ai.key,
            donor_vault.key,
            &[donor_vault.key],
        )?,
        &[
            token_program.clone(),
            donor_vault.clone(),
            destination_mint2_ai.clone(),
            user_ai.clone(),
        ],
        &[&[user_ai.key.as_ref(), mint2.key.as_ref(), &[bump2]]],
    )?;

    **user_ai.try_borrow_mut_lamports()? = user_ai
        .lamports()
        .checked_add(eb_ai.lamports())
        .ok_or(ProgramError::InsufficientFunds)? //todo find better error
        .checked_add(oracle.lamports())
        .ok_or(ProgramError::InsufficientFunds)?; //todo find better error
    *eb_ai.try_borrow_mut_data()? = &mut [];
    **eb_ai.try_borrow_mut_lamports()? = 0;
    *oracle.try_borrow_mut_data()? = &mut [];
    **oracle.try_borrow_mut_lamports()? = 0;

    Ok(())
}
