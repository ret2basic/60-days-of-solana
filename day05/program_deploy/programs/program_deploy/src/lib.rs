use anchor_lang::prelude::*;

declare_id!("6mV3DoqKv4Bo37wtXaXa5wEDfpSGUZNCgDaZVz9cKNtH");

#[program]
pub mod program_deploy {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("program_deploy version 2");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
