use {
    carbon_core::{borsh, CarbonDeserialize},
    spl_token_2022::state,
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum AccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

impl From<state::AccountState> for AccountState {
    fn from(state: state::AccountState) -> Self {
        match state {
            state::AccountState::Uninitialized => Self::Uninitialized,
            state::AccountState::Initialized => Self::Initialized,
            state::AccountState::Frozen => Self::Frozen,
        }
    }
}
