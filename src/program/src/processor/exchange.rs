use crate::helpers::convert_to_u64;
use crate::state::OracleAccount;
use borsh::BorshDeserialize;
use core::panic;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::{instruction::transfer, state::Account};

const BOOTH_SPREAD: f64 = 0.05;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    deposited_amount: f64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let user_ai = next_account_info(accounts_iter)?;

    let receiver_vault = next_account_info(accounts_iter)?;
    let donor_vault = next_account_info(accounts_iter)?;

    let receiver_account = next_account_info(accounts_iter)?;
    let donor_account = next_account_info(accounts_iter)?;

    let oracle_ai = next_account_info(accounts_iter)?;

    let token_program = next_account_info(accounts_iter)?;

    let oracle = OracleAccount::try_from_slice(&oracle_ai.data.borrow())?;
    let rate = oracle.exchange_rate;

    let donor_account_content = Account::unpack(&donor_account.data.borrow())?;
    let receiver_account_content = Account::unpack(&receiver_account.data.borrow())?;

    msg!("donor_account: {:?}", donor_account_content);
    msg!("receiver_account_content: {:?}", receiver_account_content);
    msg!("rate: {:?}", rate);
    msg!("deposited_amount: {:?}", deposited_amount);

    let donor_mint = donor_account_content.mint;
    let receivier_mint = receiver_account_content.mint;

    let (oracle_receiver_to_donor_key, _bump) = Pubkey::find_program_address(
        &[
            user_ai.key.as_ref(),
            receivier_mint.as_ref(),
            donor_mint.as_ref(),
        ],
        program_id,
    );

    let (oracle_donor_to_receiver_key, _bump) = Pubkey::find_program_address(
        &[
            user_ai.key.as_ref(),
            donor_mint.as_ref(),
            receivier_mint.as_ref(),
        ],
        program_id,
    );

    let (donor_vault_key, donor_vault_bump) =
        Pubkey::find_program_address(&[user_ai.key.as_ref(), receivier_mint.as_ref()], program_id);

    msg!("________donor_vault_key : {:?}", donor_vault_key);

    let ff = oracle_ai.key;
    let withdrawn_tokens: u64;

    msg!("oracle key: {:?}", oracle_ai.key);
    msg!(
        "oracle_receiver_to_donor_key: {:?}",
        oracle_receiver_to_donor_key
    );
    msg!(
        "oracle_donor_to_receiver_key: {:?}",
        oracle_donor_to_receiver_key
    );
    msg!("receivier_mint: {:?}", receivier_mint);
    msg!(
        "oracle_donor_to_receiver_key: {:?}",
        oracle_donor_to_receiver_key
    );

    if ff == &oracle_receiver_to_donor_key {
        withdrawn_tokens =
            convert_to_u64(oracle.exchange_rate * deposited_amount * (1.0 - BOOTH_SPREAD));
    } else if ff == &oracle_donor_to_receiver_key {
        withdrawn_tokens =
            convert_to_u64(deposited_amount * (1.0 - BOOTH_SPREAD) / oracle.exchange_rate);
    } else {
        panic!("incorrect oracle key");
    }

    let deposited_tokens = convert_to_u64(deposited_amount);

    invoke(
        &transfer(
            token_program.key,
            donor_account.key,
            receiver_vault.key,
            user_ai.key,
            &[user_ai.key],
            deposited_tokens,
        )?,
        &[
            token_program.clone(),
            receiver_vault.clone(),
            donor_account.clone(),
            user_ai.clone(),
        ],
    )?;

    invoke_signed(
        &transfer(
            token_program.key,
            donor_vault.key,
            receiver_account.key,
            donor_vault.key,
            &[donor_vault.key],
            withdrawn_tokens,
        )?,
        &[
            token_program.clone(),
            donor_vault.clone(),
            receiver_account.clone(),
            user_ai.clone(),
        ],
        &[&[
            user_ai.key.as_ref(),
            receivier_mint.as_ref(),
            &[donor_vault_bump],
        ]],
    )?;

    Ok(())
}
