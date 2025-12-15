use crate::state::status::ProductStatus;
use anchor_lang::prelude::*;

#[account]
pub struct ProductRegistry {
    pub product_pubkey: Pubkey,
    pub auther_pubkey: Pubkey,
    pub owner_pubkey: Pubkey,
    pub metadata_hash: [u8; 32],
    pub creation_date: u64,
    pub status: ProductStatus,
    pub bump: u8,
}

impl ProductRegistry {
    const PUBKEYSIZES: usize = 3 * 32; // product_pubkey + auther_pubkey + owner_pubkey
    const METADATASIZE: usize = 32; // metadata_hash
    const CREATIONDATESIZE: usize = 8; // creation_date
    const PRODUCTSTATUSSIZE: usize = 1; // status
    const BUMPSIZE: usize = 1; // bump

    pub const LEN: usize = usize::checked_add(Self::PUBKEYSIZES, Self::METADATASIZE)
        .unwrap()
        .checked_add(Self::CREATIONDATESIZE)
        .unwrap()
        .checked_add(Self::PRODUCTSTATUSSIZE)
        .unwrap()
        .checked_add(Self::BUMPSIZE)
        .unwrap();
}
