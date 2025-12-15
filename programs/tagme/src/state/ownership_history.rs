use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
pub struct OwnershipHistory {
    pub product_pubkey: Pubkey,
    pub previous_owners: [Pubkey; 5],
    pub last_transfer_time: u64,
    pub bump: u8,
}

impl OwnershipHistory {
    const PUBKEYSIZES: usize = 6 * 32;
    const TIMESIZE: usize = 8;
    const BUMPSIZE: usize = 1;

    pub const LEN: usize = Self::PUBKEYSIZES
        .checked_add(Self::TIMESIZE)
        .unwrap()
        .checked_add(Self::BUMPSIZE)
        .unwrap();
}
