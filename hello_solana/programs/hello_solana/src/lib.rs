use anchor_lang::prelude::*;

declare_id!("7qgm6MkpAUNkWvT3k6WVZz4bBW1WTY1YMTQBLpQNPvSj");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
