use crate::{
    error::TagMeError,
    state::{ProductRegistry, ProductStatus, PRODUCT_ACC_SEED},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(product_pubkey: Pubkey)]
pub struct RevokeProduct<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [PRODUCT_ACC_SEED, product_pubkey.as_ref()],
        bump = product_acc.bump,
    )]
    pub product_acc: Account<'info, ProductRegistry>,
}

pub fn handler(ctx: Context<RevokeProduct>) -> Result<()> {
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

    product.status = ProductStatus::Revoked;

    Ok(())
}
