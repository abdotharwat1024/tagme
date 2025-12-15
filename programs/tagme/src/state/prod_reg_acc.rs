use crate::state::status::ProductStatus;
use anchor_lang::prelude::Pubkey;

pub struct ProductRegistery {
    product_pubkey: Pubkey,
    auther_pubkey: Pubkey,
    owner_pubkey: Pubkey,
    metadata_hash: [u8; 32],
    creation_date: u64,
    status: ProductStatus,
    bump: u8,
}
