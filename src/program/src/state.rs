use borsh::{BorshDeserialize, BorshSerialize};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExchangeBoothAccount {
    pub fee: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OracleAccount {
    pub exchange_rate: u64,
    pub decimals: u8,
}
