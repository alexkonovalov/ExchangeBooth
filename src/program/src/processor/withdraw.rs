use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{instruction::transfer, state::Account};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_ai = next_account_info(accounts_iter)?;
    let vault1 = next_account_info(accounts_iter)?;
    let vault2 = next_account_info(accounts_iter)?;
    let receiver1 = next_account_info(accounts_iter)?;
    let receiver2 = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    let vault1_content = Account::unpack(&vault1.data.borrow())?;
    let vault2_content = Account::unpack(&vault2.data.borrow())?;

    let (_vault1_key, vault1_bump) = Pubkey::find_program_address(
        &[user_ai.key.as_ref(), vault1_content.mint.as_ref()],
        program_id,
    );

    let (_vault2_key, vault2_bump) = Pubkey::find_program_address(
        &[user_ai.key.as_ref(), vault2_content.mint.as_ref()],
        program_id,
    );

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
            user_ai.key.as_ref(),
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
            user_ai.key.as_ref(),
            vault2_content.mint.as_ref(),
            &[vault2_bump],
        ]],
    )?;

    Ok(())
}
