use anchor_lang::prelude::*;

declare_id!("3Uuxn1oKYE9c9WAvrd9MQseqPyjtHRvfJHLr6wj3ERvX");
#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn increment(ctx: Context<Operate>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_add(1).unwrap();
        Ok(())
    }
    pub fn decrement(ctx: Context<Operate>) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter.count.checked_sub(1).unwrap();
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
        payer = payer
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operate<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    count: u64,
}
