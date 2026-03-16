use anchor_lang::prelude::*;

pub mod calculate;

declare_id!("45XGNnwXwFGzpMDwPrDy7iy1NGSigV4GcT1nGwVqdCEH");

#[program]
pub mod visibility_lab {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.file_private_value = 0;
        state.internal_value = 0;
        state.internal_from_other_module_value = 0;
        state.restricted_value = 0;
        state.separate_file_sum = 0;
        state.inline_sum = 0;

        msg!("visibility lab initialized for {:?}", state.authority);
        Ok(())
    }

    pub fn file_private_demo(ctx: Context<UpdateState>) -> Result<()> {
        let value = get_a_num();
        ctx.accounts.state.file_private_value = value;

        msg!("file-private helper returned {}", value);
        Ok(())
    }

    pub fn internal_visibility_demo(ctx: Context<UpdateState>) -> Result<()> {
        let direct = some_internal_function::internal_function();
        let from_other_module = external_views::call_internal_from_outside_program_module();

        ctx.accounts.state.internal_value = direct;
        ctx.accounts.state.internal_from_other_module_value = from_other_module;

        msg!("internal function from handler = {}", direct);
        msg!("internal function from another module = {}", from_other_module);
        Ok(())
    }

    pub fn restricted_visibility_demo(ctx: Context<UpdateState>) -> Result<()> {
        let value = some_private_function::private_function();
        ctx.accounts.state.restricted_value = value;

        msg!("restricted function from parent module = {}", value);
        Ok(())
    }

    pub fn separate_file_module_demo(
        ctx: Context<UpdateState>,
        x: u64,
        y: u64,
    ) -> Result<()> {
        let result = calculate::add(x, y);
        ctx.accounts.state.separate_file_sum = result;

        msg!("separate file add: {} + {} = {}", x, y, result);
        Ok(())
    }

    pub fn inline_module_demo(ctx: Context<UpdateState>, x: u64, y: u64) -> Result<()> {
        let result = inline_calculate::add(x, y);
        ctx.accounts.state.inline_sum = result;

        msg!("inline module add: {} + {} = {}", x, y, result);
        Ok(())
    }

    pub mod some_internal_function {
        pub fn internal_function() -> u64 {
            7
        }
    }

    pub mod some_private_function {
        pub(in crate::visibility_lab) fn private_function() -> u64 {
            13
        }
    }
}

fn get_a_num() -> u64 {
    2
}

mod external_views {
    use crate::visibility_lab;

    pub fn call_internal_from_outside_program_module() -> u64 {
        visibility_lab::some_internal_function::internal_function()
    }
}

mod inline_calculate {
    pub fn add(x: u64, y: u64) -> u64 {
        x + y
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + VisibilityState::INIT_SPACE)]
    pub state: Account<'info, VisibilityState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, VisibilityState>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct VisibilityState {
    pub authority: Pubkey,
    pub file_private_value: u64,
    pub internal_value: u64,
    pub internal_from_other_module_value: u64,
    pub restricted_value: u64,
    pub separate_file_sum: u64,
    pub inline_sum: u64,
}
