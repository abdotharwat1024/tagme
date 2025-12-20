use crate::state::MAX_HISTORY;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
pub struct OwnershipHistory {
    pub product_pubkey: Pubkey,
    pub previous_owners: [Pubkey; MAX_HISTORY],
    pub last_transfer_time: u64,
    pub head: u8,
    pub bump: u8,
}

impl OwnershipHistory {
    const PUBKEYSIZES: usize = 32 + (5 * 32);
    const TIMESIZE: usize = 8;
    const BUMPSIZE: usize = 1;

    pub const INIT_SPACE: usize = Self::PUBKEYSIZES
        .checked_add(Self::TIMESIZE)
        .unwrap()
        .checked_add(Self::BUMPSIZE)
        .unwrap();
}
