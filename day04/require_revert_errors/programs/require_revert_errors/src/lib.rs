use anchor_lang::prelude::*;

declare_id!("GVR4ikhXJhxQU4CPDqMyijqnnBkHFu8KWJMYiQqW9Uy6");

#[program]
pub mod require_revert_errors {
    use super::*;

    pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, Day4Error::AisTooSmall);
        require!(a <= 100, Day4Error::AisTooBig);

        msg!("Result = {}", a);
        Ok(())
    }

    pub fn func(_ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        err!(Day4Error::AlwaysErrors)
    }

    pub fn func_ok(_ctx: Context<ReturnOk>) -> Result<()> {
        msg!("Will this print?");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LimitRange {}

#[derive(Accounts)]
pub struct ReturnOk {}

#[error_code]
pub enum Day4Error {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")]
    AlwaysErrors,
}
