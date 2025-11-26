use anchor_lang::prelude::Pubkey;


pub struct User{
    user_pubkey: Pubkey,
    name: String,
    url: String,
    bump: u8
}