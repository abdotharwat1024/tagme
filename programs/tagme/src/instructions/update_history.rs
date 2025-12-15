use anchor_lang::prelude::*;

use crate::state::{OwnershipHistory, ProductRegistry};
pub const PRODUCT_HISTORY_SEED: &[u8] = b"history";

#[derive(Accounts)]
struct UpdateHistory<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(mut, seeds=[PRODUCT_HISTORY_SEED, product_acc.key().as_ref()], bump)]
    history_acc: Account<'info, OwnershipHistory>,
    product_acc: Account<'info, ProductRegistry>,
}

pub fn handler(ctx: Context<UpdateHistory>, new_owner: Pubkey) -> Result<()> {
    Ok(())
}
