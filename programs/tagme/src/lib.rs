use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;

use instructions::*;

declare_id!("GzbdrVL98EEGWEdTorx6oiNKrR4PsmhkQ6HKYNnNepd5");

#[program]
pub mod tagme {
    use super::*;
    pub fn register_product(
        ctx: Context<RegisterProduct>,
        product_pubkey: Pubkey,
        metadata: [u8; 32],
    ) -> Result<()> {
        register_product::handler(ctx, product_pubkey, metadata)
    }

    pub fn register_user(ctx: Context<RegisterUser>, name: String, url: String) -> Result<()> {
        register_user::handler(ctx, name, url)
    }

    pub fn revoke_product(ctx: Context<RevokeProduct>, product_pubkey: Pubkey) -> Result<()> {
        revoke_product::handler(ctx)
    }

    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        product_pubkey: Pubkey,
        new_owner: Pubkey,
    ) -> Result<()> {
        transfer_ownership::handler(ctx, new_owner)
    }

    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        product_pubkey: Pubkey,
        new_hash: [u8; 32],
    ) -> Result<()> {
        update_metadata::handler(ctx, new_hash)
    }
}
