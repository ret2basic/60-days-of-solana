# Day 12

This day is complete in `day12/sysvars_lab`.

## What was done

- Created an Anchor project named `sysvars_lab`
- Adapted the article into a runnable sysvar snapshot lab that stores values in one account instead of relying only on program logs
- Covered the sysvars that are practical in the current toolchain: `Clock`, `EpochSchedule`, and `Rent` via `get()`, plus `StakeHistory`, `Instructions`, and `LastRestartSlot` via sysvar accounts passed into the instruction
- Added TypeScript tests that verify the persisted snapshot against the local validator and the client connection

## Commands that worked here

```bash
cd day12/sysvars_lab
yarn install
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `5zwnS8QfQU7ECiWd6wJtVTQSeugKqTuMvC2uuqhJhUb5`
- `initialize(number)` snapshots several sysvars into `SysvarSnapshot` so tests can assert values directly
- `Clock::get()` stores `unix_timestamp`, `slot`, `epoch`, `epoch_start_timestamp`, and `leader_schedule_epoch`
- `EpochSchedule::get()` stores `slots_per_epoch`, `leader_schedule_slot_offset`, `warmup`, `first_normal_epoch`, and `first_normal_slot`
- `Rent::get()` stores `lamports_per_byte_year`, a scaled `exemption_threshold` value, `burn_percent`, and the rent-exempt minimum for a zero-byte account
- `StakeHistory::from_account_info(...)` reads the stake-history sysvar account; on a local validator it is expected to contain zero historical entries
- `instructions::load_instruction_at_checked(...)` plus `load_current_index_checked(...)` are used to inspect the current transaction and recover the `number` argument from serialized instruction data
- `LastRestartSlot::from_account_info(...)` is used with the article's custom public-key path, even though this toolchain also supports `LastRestartSlot::get()` directly
- Deprecated or unsupported sysvars from the article are not reimplemented here because day11 already covered `RecentBlockhashes`, and `Fees`, `EpochRewards`, `SlotHistory`, and `SlotHashes` are not useful targets in the current Anchor flow

## Exercise Answer

- `LastRestartSlot` can be accessed by passing the `SysvarLastRestartS1ot1111111111111111111111` account into the instruction and decoding it with `LastRestartSlot::from_account_info(...)`. In the current SDK it also supports `LastRestartSlot::get()`, but this day keeps the account-based approach because that is what the article asks the reader to practice.