use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub user_pubkey: Pubkey,
    pub name: String,
    pub url: String,
    pub bump: u8,
}

impl User {
    const BUMPSIZE: usize = 1; // bump
    pub const MAX_NAME_LEN: usize = 32;
    pub const MAX_URL_LEN: usize = 64;
    pub const LEN: usize = 8_usize // user_pubkey: Pubkey
        .checked_add(Self::MAX_NAME_LEN)
        .unwrap()
        .checked_add(Self::MAX_URL_LEN)
        .unwrap()
        .checked_add(Self::BUMPSIZE)
        .unwrap();
}
