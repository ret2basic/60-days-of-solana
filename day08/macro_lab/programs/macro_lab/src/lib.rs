use anchor_lang::prelude::*;

declare_id!("6cWAZo6vX3aNxRkWAfq3fvJyjQj1NNDraXeAQuWx7562");

#[program]
pub mod macro_lab {
    use super::*;

    pub fn function_like_macro_demo(ctx: Context<Initialize>) -> Result<()> {
        msg!("program id = {:?}", ctx.program_id);
        msg!("macro accepts one value: {}", 1);
        msg!("macro accepts many values: {} {} {}", 1, 2, 3);

        let formatted = format!("format! also expands code: {} + {} = {}", 2, 3, 5);
        msg!("{}", formatted);

        Ok(())
    }

    pub fn fixed_arity_function_demo(_ctx: Context<Initialize>) -> Result<()> {
        msg!("one arg function = {}", write_one(b"Hello, world!"));
        msg!("two arg function = {}", write_two(b"Hello", b"macro"));
        msg!("three arg function = {}", write_three(b"Rust", b"macros", b"expand"));
        Ok(())
    }

    pub fn anchor_macro_inventory(_ctx: Context<Initialize>) -> Result<()> {
        msg!("#[program] is an attribute-like macro");
        msg!("#[derive(Accounts)] is a custom derive macro");
        msg!("msg! is a function-like macro");
        Ok(())
    }
}

fn write_one(arg1: &[u8]) -> String {
    String::from_utf8(arg1.to_vec()).unwrap()
}

fn write_two(arg1: &[u8], arg2: &[u8]) -> String {
    String::from_utf8([arg1, b" ", arg2].concat()).unwrap()
}

fn write_three(arg1: &[u8], arg2: &[u8], arg3: &[u8]) -> String {
    String::from_utf8([arg1, b" ", arg2, b" ", arg3].concat()).unwrap()
}

#[derive(Accounts)]
pub struct Initialize {}
