use anchor_lang::prelude::*;

declare_id!("2MsFAjeMf2ddQxUNrfqeH7m1r5cTFaJhtgS5k7Lg3St1");

#[program]
pub mod arithmetic_basics {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn add(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        msg!("Result = {}", a + b);
        Ok(())
    }

    pub fn subtract(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_sub(b).ok_or(MathError::Underflow)?;
        msg!("Result = {}", result);
        Ok(())
    }

    pub fn multiply(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a.checked_mul(b).ok_or(MathError::Overflow)?;
        msg!("Result = {}", result);
        Ok(())
    }

    pub fn divide(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        require!(b != 0, MathError::DivisionByZero);
        msg!("Result = {}", a / b);
        Ok(())
    }

    pub fn power(_ctx: Context<Initialize>, base: u64, exponent: u32) -> Result<()> {
        let result = base.checked_pow(exponent).ok_or(MathError::Overflow)?;
        msg!("Result = {}", result);
        Ok(())
    }

    pub fn cube_root(_ctx: Context<Initialize>, value: f64) -> Result<()> {
        msg!("attempting to begin the function with {}", value);
        msg!("Result = {}", value.cbrt());
        Ok(())
    }

    pub fn sqrt(_ctx: Context<Initialize>, value: f64) -> Result<()> {
        require!(value >= 0.0, MathError::NegativeInput);
        msg!("Result = {}", value.sqrt());
        Ok(())
    }

    pub fn log10(_ctx: Context<Initialize>, value: f64) -> Result<()> {
        require!(value > 0.0, MathError::NonPositiveLogInput);
        msg!("Result = {}", value.log10());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum MathError {
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Square root requires a non-negative input")]
    NegativeInput,
    #[msg("Log10 requires a positive input")]
    NonPositiveLogInput,
}
