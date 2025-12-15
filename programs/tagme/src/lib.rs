use anchor_lang::prelude::*;
mod error;
mod instructions;
mod state;

declare_id!("GzbdrVL98EEGWEdTorx6oiNKrR4PsmhkQ6HKYNnNepd5");

#[program]
pub mod tagme {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
