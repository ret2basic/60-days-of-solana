use anchor_lang::prelude::*;

declare_id!("BY8LCvSGVzA61LsuEkBv41etvWTcopxWd8K14cAeXj7Y");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(_ctx: Context<Empty>, first_arg: u64) -> Result<()> {
        msg!("Boaty received {}", first_arg);
        Ok(())
    }

    pub fn add(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let sum = a.checked_add(b).ok_or(MathError::Overflow)?;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let difference = a.checked_sub(b).ok_or(MathError::Underflow)?;
        msg!("Difference is {}", difference);
        Ok(())
    }

    pub fn mul(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let product = a.checked_mul(b).ok_or(MathError::Overflow)?;
        msg!("Product is {}", product);
        Ok(())
    }

    pub fn div(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        require!(b != 0, MathError::DivisionByZero);
        msg!("Quotient is {}", a / b);
        Ok(())
    }

    pub fn modulo(_ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        require!(b != 0, MathError::DivisionByZero);
        msg!("Modulo is {}", a % b);
        Ok(())
    }

    pub fn non_empty_account_example(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        msg!("Primary signer {}", ctx.accounts.signer.key());
        msg!("Secondary signer {}", ctx.accounts.another_signer.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[error_code]
pub enum MathError {
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Division by zero")]
    DivisionByZero,
}
