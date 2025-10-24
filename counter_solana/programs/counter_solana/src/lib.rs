use anchor_lang::prelude::*;

declare_id!("5oPatdoYFYVUknLwTfy2jEZfZ9cvGHkSXdxRPs2EKCpG");

#[program]
pub mod counter_solana {
    use super::*;

    pub fn initialize_counter(
        ctx: Context<InitializeCounter>,
        max_value: u64,
        min_value: u64,
    ) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require_gt!(max_value, min_value, CounterError::InvalidArguments);
        counter.count = min_value;
        counter.max_value = max_value;
        counter.min_value = min_value;
        msg!("Counter initialized");
        Ok(())
    }

    pub fn increment(ctx: Context<Operate>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Check if counter is initialized
        require_gt!(counter.max_value, 0, CounterError::NotInitializedError);
        
        // Check bounds BEFORE updating the counter
        require_gt!(
            counter.max_value,
            counter.count,
            CounterError::MaxValueError
        );
        
        // Safe increment with overflow protection
        counter.count = counter.count.checked_add(1)
            .ok_or(CounterError::MaxValueError)?;
        
        let payer = &ctx.accounts.signer.key();
        msg!("{:?}'s Counter: {:?}", payer, counter.count);
        Ok(())
    }
    pub fn decrement(ctx: Context<Operate>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Check if counter is initialized
        require_gt!(counter.max_value, 0, CounterError::NotInitializedError);
        
        // Check bounds BEFORE updating the counter
        require_gte!(
            counter.count,
            counter.min_value,
            CounterError::MinValueError
        );
        
        // Safe decrement with underflow protection
        counter.count = counter.count.checked_sub(1)
            .ok_or(CounterError::MinValueError)?;
        
        let payer = &ctx.accounts.signer.key();
        msg!("{:?}'s Counter: {:?}", payer, counter.count);
        Ok(())
    }

    pub fn reset(ctx: Context<Operate>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.min_value;
        msg!("Counter reset");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"Counter",payer.key().as_ref()],
        bump,
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operate<'info> {
    #[account(mut,
    seeds = [b"Counter",signer.key().as_ref()],
    bump,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
    max_value: u64,
    min_value: u64,
}

#[error_code]
pub enum CounterError {
    #[msg("Counter is not yet initialized")]
    NotInitializedError,
    #[msg("Counter at its maximum")]
    MaxValueError,
    #[msg("Counter at its minimum")]
    MinValueError,
    #[msg("Arguments are not valid")]
    InvalidArguments,
}
