# Day 11

This day is complete in `day11/sysvar_lab`.

## What was done

- Created an Anchor project named `sysvar_lab`
- Adapted the article into a small sysvar lab that records clock fields, derives the day of the week from `unix_timestamp`, and reads the deprecated recent blockhash sysvar
- Added one state account so tests can verify the values directly instead of relying only on transaction logs
- Added TypeScript tests covering all 3 day11 behaviors against a local validator

## Commands that worked here

```bash
cd day11/sysvar_lab
yarn install
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `Fgun4nxa8e5feH7s4KygAJYPfb92sZv3MYnLdS5FrdeK`
- `initialize()` reads `Clock::get()` and stores `unix_timestamp`, `slot`, `epoch`, `epoch_start_timestamp`, and `leader_schedule_epoch`
- `get_day_of_the_week()` uses the `chrono` crate to convert the current `unix_timestamp` into a `NaiveDateTime` and then stores the weekday string
- `read_recent_blockhash()` follows the article's deprecated sysvar pattern with `RecentBlockhashes::from_account_info(...)` and `SYSVAR_RECENT_BLOCKHASHES_PUBKEY`
- The recent blockhash section is intentionally labeled deprecated in the code comments and docs because it still exists in this toolchain but is not the forward-looking Solana approach
- The article says block number is not a 1:1 analog in Solana; this lab still records `slot` from the clock sysvar because it is the closest runtime value discussed for that comparison

## Exercise Answers

- `block.timestamp` analog: use `Clock::get()?.unix_timestamp`. In this lab it is stored during `initialize()` and logged immediately.
- Day of week: convert `unix_timestamp` with `chrono::NaiveDateTime::from_timestamp_opt(...)` and call `.weekday()`.
- `block.number` analog: Solana uses slots rather than Ethereum block numbers. This lab records `clock.slot` as the closest runtime value.
- Recent blockhash: the deprecated `RecentBlockhashes` sysvar can still be read by passing the sysvar account and decoding it with `from_account_info(...)`.
- Missing Solidity analogs: there is no in-program equivalent for miner coinbase, chain id, block difficulty, or dynamic base fee in the same Solidity sense on Solana.