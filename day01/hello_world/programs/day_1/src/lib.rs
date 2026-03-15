use anchor_lang::prelude::*;

declare_id!("7L76AwhoSX32paoD4Up7rYV4f9tySw5JxhKxFFRNeUBc");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
