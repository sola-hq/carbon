use {
    carbon_core::{borsh, CarbonDeserialize},
    solana_program::program_option::COption,
    spl_token_2022::state,
};
#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6a5edd53c00a4a4a")]
pub struct Mint {
    pub mint_authority: Option<solana_pubkey::Pubkey>,
    pub supply: u64,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<solana_pubkey::Pubkey>,
}

impl From<state::Mint> for Mint {
    fn from(mint: state::Mint) -> Self {
        Self {
            mint_authority: match mint.mint_authority {
                COption::Some(pubkey) => Some(pubkey),
                COption::None => None,
            },
            supply: mint.supply,
            decimals: mint.decimals,
            is_initialized: mint.is_initialized,
            freeze_authority: match mint.freeze_authority {
                COption::Some(pubkey) => Some(pubkey),
                COption::None => None,
            },
        }
    }
}
