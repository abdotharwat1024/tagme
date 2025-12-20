use crate::{
    error::TagMeError,
    state::{User, USER_ACC_SEED},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [USER_ACC_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = User::INIT_SPACE + 8,
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterUser>, name: String, url: String) -> Result<()> {
    require!(name.len() <= User::MAX_NAME_LEN, TagMeError::InvalidName);
    require!(url.len() <= User::MAX_URL_LEN, TagMeError::InvalidUrl);

    let user = &mut ctx.accounts.user;

    user.name = name;
    user.url = url;
    user.user_pubkey = ctx.accounts.authority.key();
    user.bump = ctx.bumps.user;

    Ok(())
}
