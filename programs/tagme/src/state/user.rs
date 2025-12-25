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
    pub const MAX_NAME_LEN: usize = 32 + 4; // String prefix + max length
    pub const MAX_URL_LEN: usize = 64 + 4; // String prefix + max length
    pub const INIT_SPACE: usize = 8_usize // user_pubkey: Pubkey
        .checked_add(Self::MAX_NAME_LEN)
        .unwrap()
        .checked_add(Self::MAX_URL_LEN)
        .unwrap()
        .checked_add(Self::BUMPSIZE)
        .unwrap();
}
