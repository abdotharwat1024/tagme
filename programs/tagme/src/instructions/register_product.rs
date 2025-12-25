use crate::error::TagMeError;
use crate::state::{OwnershipHistory, ProductRegistry, ProductStatus};
use crate::state::{PRODUCT_ACC_SEED, PRODUCT_HISTORY_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(product_pubkey: Pubkey)]
pub struct RegisterProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [PRODUCT_ACC_SEED, product_pubkey.as_ref()],
        bump,
        space = ProductRegistry::INIT_SPACE + 8,
    )]
    pub product: Account<'info, ProductRegistry>,
    #[account(
        init,
        payer = authority,
        seeds = [PRODUCT_HISTORY_SEED, product_pubkey.as_ref()],
        bump,
        space = OwnershipHistory::INIT_SPACE + 8,
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
    let now = Clock::get()?.unix_timestamp as u64;

    require!(
        product_pubkey != Pubkey::default(),
        TagMeError::InvalidAccount
    );

    product.product_pubkey = product_pubkey;
    product.author_pubkey = ctx.accounts.authority.key();
    product.owner_pubkey = ctx.accounts.authority.key();
    product.creation_date = now;
    product.metadata_hash = metadata;
    product.bump = ctx.bumps.product;
    product.status = ProductStatus::Active;

    product_history.product_pubkey = product.product_pubkey;
    product_history.last_transfer_time = now;
    product_history.head = 0;
    product_history.bump = ctx.bumps.product_history;

    Ok(())
}
