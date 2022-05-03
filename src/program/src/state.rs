use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExchangeBoothAccount {
    pub vault1: Pubkey,
    pub vault2: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OracleAccount {
    pub exchange_rate: f64,
}
