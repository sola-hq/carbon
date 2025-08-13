use {
    super::Token2022Decoder,
    crate::PROGRAM_ID,
    carbon_core::account::AccountDecoder,
    solana_program_pack::Pack,
    spl_token_2022::{extension::StateWithExtensions, state},
};

pub mod mint;
pub mod multisig;
pub mod token;

pub enum Token2022Account {
    Mint(mint::Mint),
    Token(token::Token),
    Multisig(multisig::Multisig),
}

impl AccountDecoder<'_> for Token2022Decoder {
    type AccountType = Token2022Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Ok(decoded_account) = StateWithExtensions::<state::Account>::unpack(&account.data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
                data: Token2022Account::Token(decoded_account.base.into()),
            });
        }

        if let Ok(decoded_account) = StateWithExtensions::<state::Mint>::unpack(&account.data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
                data: Token2022Account::Mint(decoded_account.base.into()),
            });
        }

        if let Ok(decoded_account) = state::Multisig::unpack_unchecked(&account.data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
                data: Token2022Account::Multisig(decoded_account.into()),
            });
        }

        None
    }
}
