use crate::state::{ProductRegistry, ProductStatus};
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
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterProduct>,
    product_pubkey: Pubkey,
    metadata: [u8; 32],
) -> Result<()> {
    let prod = &mut ctx.accounts.product;

    prod.product_pubkey = product_pubkey;
    prod.auther_pubkey = ctx.accounts.authority.key();
    prod.owner_pubkey = ctx.accounts.authority.key();
    prod.creation_date = Clock::get()?.unix_timestamp as u64;
    prod.metadata_hash = metadata;
    prod.bump = ctx.bumps.product;
    prod.status = ProductStatus::ACTIVE;

    Ok(())
}
