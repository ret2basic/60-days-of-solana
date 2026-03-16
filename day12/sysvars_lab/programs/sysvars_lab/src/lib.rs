use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{
        instructions,
        last_restart_slot::LastRestartSlot,
        stake_history::StakeHistory,
    },
};

declare_id!("5zwnS8QfQU7ECiWd6wJtVTQSeugKqTuMvC2uuqhJhUb5");

#[program]
pub mod sysvars_lab {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, number: u32) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let clock = Clock::get()?;
        let epoch_schedule = EpochSchedule::get()?;
        let rent = Rent::get()?;
        let stake_history = StakeHistory::from_account_info(&ctx.accounts.stake_history)?;
        let instruction_details =
            instructions::load_instruction_at_checked(0, &ctx.accounts.instruction_sysvar)?;
        let current_instruction_index =
            instructions::load_current_index_checked(&ctx.accounts.instruction_sysvar)?;
        let last_restart_slot =
            LastRestartSlot::from_account_info(&ctx.accounts.last_restart_slot)?;

        require!(
            instruction_details.data.len() >= 12,
            Day12Error::InstructionDataTooShort
        );

        let observed_number = u32::from_le_bytes(
            instruction_details.data[instruction_details.data.len() - 4..]
                .try_into()
                .map_err(|_| error!(Day12Error::InstructionDataTooShort))?,
        );

        require!(
            observed_number == number,
            Day12Error::InstructionArgumentMismatch
        );

        state.authority = ctx.accounts.authority.key();
        state.unix_timestamp = clock.unix_timestamp;
        state.slot = clock.slot;
        state.epoch = clock.epoch;
        state.epoch_start_timestamp = clock.epoch_start_timestamp;
        state.leader_schedule_epoch = clock.leader_schedule_epoch;
        state.slots_per_epoch = epoch_schedule.slots_per_epoch;
        state.leader_schedule_slot_offset = epoch_schedule.leader_schedule_slot_offset;
        state.warmup = epoch_schedule.warmup;
        state.first_normal_epoch = epoch_schedule.first_normal_epoch;
        state.first_normal_slot = epoch_schedule.first_normal_slot;
        state.lamports_per_byte_year = rent.lamports_per_byte_year;
        state.rent_exemption_threshold_milli = (rent.exemption_threshold * 1000.0).round() as u64;
        state.burn_percent = u64::from(rent.burn_percent);
        state.minimum_balance_for_zero_bytes = rent.minimum_balance(0);
        state.stake_history_entries = stake_history.iter().count() as u32;
        state.instruction_program_id = instruction_details.program_id;
        state.instruction_account_count = instruction_details.accounts.len() as u32;
        state.instruction_data_len = instruction_details.data.len() as u32;
        state.current_instruction_index = u64::from(current_instruction_index);
        state.provided_number = number;
        state.observed_number = observed_number;
        state.last_restart_slot = last_restart_slot.last_restart_slot;

        msg!("Clock slot: {}", clock.slot);
        msg!("Epoch schedule slots/epoch: {}", epoch_schedule.slots_per_epoch);
        msg!("Rent lamports/byte-year: {}", rent.lamports_per_byte_year);
        msg!("Stake history entries available: {}", state.stake_history_entries);
        msg!("Instruction observed number: {}", observed_number);
        msg!("Last restart slot: {}", state.last_restart_slot);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + SysvarSnapshot::INIT_SPACE)]
    pub state: Account<'info, SysvarSnapshot>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: read-only stake history sysvar account provided by the transaction
    pub stake_history: AccountInfo<'info>,
    /// CHECK: read-only instructions sysvar account provided by the transaction
    pub instruction_sysvar: AccountInfo<'info>,
    /// CHECK: read-only last restart slot sysvar account provided by the transaction
    pub last_restart_slot: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct SysvarSnapshot {
    pub authority: Pubkey,
    pub unix_timestamp: i64,
    pub slot: u64,
    pub epoch: u64,
    pub epoch_start_timestamp: i64,
    pub leader_schedule_epoch: u64,
    pub slots_per_epoch: u64,
    pub leader_schedule_slot_offset: u64,
    pub warmup: bool,
    pub first_normal_epoch: u64,
    pub first_normal_slot: u64,
    pub lamports_per_byte_year: u64,
    pub rent_exemption_threshold_milli: u64,
    pub burn_percent: u64,
    pub minimum_balance_for_zero_bytes: u64,
    pub stake_history_entries: u32,
    pub instruction_program_id: Pubkey,
    pub instruction_account_count: u32,
    pub instruction_data_len: u32,
    pub current_instruction_index: u64,
    pub provided_number: u32,
    pub observed_number: u32,
    pub last_restart_slot: u64,
}

#[error_code]
pub enum Day12Error {
    #[msg("Instruction sysvar data was shorter than expected")]
    InstructionDataTooShort,
    #[msg("Instruction sysvar did not contain the expected initialize argument")]
    InstructionArgumentMismatch,
}
