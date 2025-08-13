use {
    carbon_core::{borsh, CarbonDeserialize},
    spl_token_2022::state,
};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032c5eecdd46ac85")]
pub struct Multisig {
    pub m: u8,
    pub n: u8,
    pub is_initialized: bool,
    pub signers: [solana_pubkey::Pubkey; 11],
}

impl From<state::Multisig> for Multisig {
    fn from(multisig: state::Multisig) -> Self {
        Self {
            m: multisig.m,
            n: multisig.n,
            is_initialized: multisig.is_initialized,
            signers: multisig.signers,
        }
    }
}
