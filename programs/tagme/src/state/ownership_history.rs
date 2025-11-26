use anchor_lang::prelude::Pubkey;

struct OwnershipHistory{
    product_pubkey: Pubkey,
    previous_owners: Vec<Pubkey>,
    last_transfer_time: u64,
    bump: u8
}