use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
    solana_program::program_option::COption,
    spl_token_2022::state,
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Token {
    pub mint: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub amount: u64,
    pub delegate: Option<solana_pubkey::Pubkey>,
    pub state: AccountState,
    pub is_native: Option<u64>,
    pub delegated_amount: u64,
    pub close_authority: Option<solana_pubkey::Pubkey>,
}

impl From<state::Account> for Token {
    fn from(account: state::Account) -> Self {
        Self {
            mint: account.mint,
            owner: account.owner,
            amount: account.amount,
            delegate: match account.delegate {
                COption::Some(key) => Some(key),
                COption::None => None,
            },
            state: account.state.into(),
            is_native: match account.is_native {
                COption::Some(v) => Some(v),
                COption::None => None,
            },
            delegated_amount: account.delegated_amount,
            close_authority: match account.close_authority {
                COption::Some(key) => Some(key),
                COption::None => None,
            },
        }
    }
}
