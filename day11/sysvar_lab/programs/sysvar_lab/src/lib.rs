use anchor_lang::{
    prelude::*,
    solana_program::account_info::next_account_info,
};

declare_id!("Fgun4nxa8e5feH7s4KygAJYPfb92sZv3MYnLdS5FrdeK");

#[program]
pub mod sysvar_lab {
    use super::*;
    use chrono::{Datelike, DateTime, Utc};

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let clock = Clock::get()?;

        state.authority = ctx.accounts.authority.key();
        state.unix_timestamp = clock.unix_timestamp;
        state.slot = clock.slot;
        state.epoch = clock.epoch;
        state.epoch_start_timestamp = clock.epoch_start_timestamp;
        state.leader_schedule_epoch = clock.leader_schedule_epoch;
        state.day_of_week = String::new();
        state.recent_blockhash = String::new();

        msg!("Block timestamp: {}", clock.unix_timestamp);
        msg!("Slot number: {}", clock.slot);
        msg!("Epoch: {}", clock.epoch);

        Ok(())
    }

    pub fn get_day_of_the_week(ctx: Context<UpdateState>) -> Result<()> {
        let clock = Clock::get()?;
        let date_time = DateTime::<Utc>::from_timestamp(clock.unix_timestamp, 0)
            .ok_or(error!(Day11Error::InvalidTimestamp))?;
        let day_of_week = date_time.weekday().to_string();

        ctx.accounts.state.day_of_week = day_of_week.clone();
        msg!("Week day is: {}", day_of_week);

        Ok(())
    }

    #[allow(deprecated)]
    pub fn read_recent_blockhash(ctx: Context<ReadRecentBlockhash>) -> Result<()> {
        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes =
            anchor_lang::solana_program::sysvar::recent_blockhashes::RecentBlockhashes::from_account_info(sysvar_info)?;
        let data = recent_blockhashes
            .last()
            .ok_or(error!(Day11Error::MissingRecentBlockhash))?;

        let blockhash = data.blockhash.to_string();
        ctx.accounts.state.recent_blockhash = blockhash.clone();

        msg!("The recent block hash is: {}", blockhash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + SysvarState::INIT_SPACE)]
    pub state: Account<'info, SysvarState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, SysvarState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReadRecentBlockhash<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, SysvarState>,
    pub authority: Signer<'info>,
    /// CHECK: read-only sysvar account used to decode deprecated recent blockhash data
    pub recent_blockhashes: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct SysvarState {
    pub authority: Pubkey,
    pub unix_timestamp: i64,
    pub slot: u64,
    pub epoch: u64,
    pub epoch_start_timestamp: i64,
    pub leader_schedule_epoch: u64,
    #[max_len(16)]
    pub day_of_week: String,
    #[max_len(64)]
    pub recent_blockhash: String,
}

#[error_code]
pub enum Day11Error {
    #[msg("Clock returned an invalid timestamp")]
    InvalidTimestamp,
    #[msg("No recent blockhash was available in the sysvar")]
    MissingRecentBlockhash,
}
