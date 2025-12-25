use anchor_lang::prelude::*;

use crate::{
    error::TagMeError,
    state::{ProductRegistry, ProductStatus, PRODUCT_ACC_SEED},
};

#[derive(Accounts)]
#[instruction(product_pubkey: Pubkey)]
pub struct UpdateMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [PRODUCT_ACC_SEED, product_pubkey.as_ref()], bump = product_acc.bump)]
    pub product_acc: Account<'info, ProductRegistry>,
}

pub fn handler(ctx: Context<UpdateMetadata>, new_hash: [u8; 32]) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let product = &mut ctx.accounts.product_acc;

    require!(
        authority.key() == product.author_pubkey,
        TagMeError::Unauthorized
    );
    require!(
        product.status == ProductStatus::Active,
        TagMeError::ProductRevoked
    );
    require!(
        product.metadata_hash != new_hash,
        TagMeError::InvalidMetadata
    );

    product.metadata_hash = new_hash;

    Ok(())
}
