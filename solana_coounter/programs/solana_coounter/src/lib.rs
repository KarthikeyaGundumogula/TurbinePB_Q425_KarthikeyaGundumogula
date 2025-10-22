use anchor_lang::prelude::*;

declare_id!("3Uuxn1oKYE9c9WAvrd9MQseqPyjtHRvfJHLr6wj3ERvX");

#[program]
pub mod solana_coounter {
    use super::*;

    pub fn init_counter(ctx: Context<InitCounter>) -> Result<()> {
        msg!("program initialized {:?}", ctx.program_id);
        ctx.accounts.counter.counter = 1;
        Ok(())
    }

    pub fn inc_counter(ctx: Context<IncCounter>) -> Result<()> {
        ctx.accounts.counter.counter += 1;
        Ok(())
    }

    pub fn dec_counter(ctx: Context<IncCounter>) -> Result<()> {
        let count = &ctx.accounts.counter.counter;
        require_gt!(*count, 1, CounterError::MinValueError);
        ctx.accounts.counter.counter -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitCounter<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"Counter"],
        bump,
        space = 8+1
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncCounter<'info> {
    #[account(
        mut,
        seeds = [b"Counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Counter {
    counter: u64,
}

#[error_code]
pub enum CounterError {
    #[msg("Counter at its minimum")]
    MinValueError,
    #[msg("Counter at its maximum")]
    MaxValueError,
}
