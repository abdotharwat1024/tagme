use crate::instructions::PRODUCT_HISTORY_SEED;
use crate::state::{OwnershipHistory, ProductRegistry, ProductStatus};
use anchor_lang::prelude::*;

const PRODUCT_ACC_SEED: &[u8] = b"product";

#[derive(Accounts)]
#[instruction(product_pubkey: Pubkey)]
pub struct RegisterProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [PRODUCT_ACC_SEED, product_pubkey.key().as_ref()],
        bump,
        space = ProductRegistry::LEN + 8,
    )]
    pub product: Account<'info, ProductRegistry>,
    #[account(
        init,
        payer = authority,
        seeds = [PRODUCT_HISTORY_SEED, product_pubkey.key().as_ref()],
        bump,
        space = OwnershipHistory::LEN + 8,
    )]
    pub product_history: Account<'info, OwnershipHistory>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterProduct>,
    product_pubkey: Pubkey,
    metadata: [u8; 32],
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    let product_history = &mut ctx.accounts.product_history;

    product.product_pubkey = product_pubkey;
    product.auther_pubkey = ctx.accounts.authority.key();
    product.owner_pubkey = ctx.accounts.authority.key();
    product.creation_date = Clock::get()?.unix_timestamp as u64;
    product.metadata_hash = metadata;
    product.bump = ctx.bumps.product;
    product.status = ProductStatus::ACTIVE;

    product_history.product_pubkey = product.product_pubkey;
    product_history.previous_owners[0] = product.product_pubkey;
    product_history.last_transfer_time = product.creation_date;
    product_history.bump = ctx.bumps.product_history;

    Ok(())
}
