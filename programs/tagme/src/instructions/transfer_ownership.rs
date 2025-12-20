use crate::error::TagMeError;
use crate::state::{
    OwnershipHistory, ProductRegistry, ProductStatus, MAX_HISTORY, PRODUCT_ACC_SEED,
    PRODUCT_HISTORY_SEED,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds=[PRODUCT_ACC_SEED, product_acc.product_pubkey.as_ref()], bump = product_acc.bump)]
    pub product_acc: Account<'info, ProductRegistry>,
    #[account(mut, seeds=[PRODUCT_HISTORY_SEED, product_acc.product_pubkey.as_ref()], bump = history_acc.bump)]
    pub history_acc: Account<'info, OwnershipHistory>,
}

pub fn handler(ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let product = &mut ctx.accounts.product_acc;
    let history = &mut ctx.accounts.history_acc;
    let idx = history.head as usize;
    require!(
        authority.key() == product.owner_pubkey,
        TagMeError::Unauthorized
    );
    require!(
        new_owner != product.owner_pubkey,
        TagMeError::InvalidAccount
    );
    require!(
        product.status == ProductStatus::Active,
        TagMeError::ProductRevoked
    );
    require!(
        product.product_pubkey == history.product_pubkey,
        TagMeError::InvalidHistory
    );

    history.previous_owners[idx] = product.owner_pubkey;
    history.head = (history.head + 1) % MAX_HISTORY as u8;
    history.last_transfer_time = Clock::get()?.unix_timestamp as u64;
    product.owner_pubkey = new_owner;

    Ok(())
}
