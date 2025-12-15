use anchor_lang::prelude::Pubkey;

pub struct ProgramState {
    admin_pubkey: Pubkey,
    version: u8,
}
