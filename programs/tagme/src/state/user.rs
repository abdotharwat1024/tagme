use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub user_pubkey: Pubkey,
    pub name: String,
    pub url: String,
    pub bump: u8,
}

impl User {
    pub const MAX_NAME_LEN: usize = 32;
    pub const MAX_URL_LEN: usize = 64;
    pub const DISCRIMINATOR_LENGTH: usize = 8;
    pub const DATA_SIZE: usize = 8_usize // user_pubkey: Pubkey
        .checked_add(
            Self::MAX_NAME_LEN, // name: String
        )
        .unwrap()
        .checked_add(
            Self::MAX_URL_LEN, // url: String,
        )
        .unwrap()
        .checked_add(
            1, // bump: u8
        )
        .unwrap();
    pub const LEN: usize = usize::checked_add(Self::DATA_SIZE, Self::DISCRIMINATOR_LENGTH).unwrap();
}
