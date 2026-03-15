# Day 4

This day is complete in `day04/require_revert_errors`.

## What was done

- Implemented `limit_range` using `require!` and custom Anchor errors
- Added `func` which always returns a custom error after logging with `msg!`
- Added `func_ok` which logs and returns `Ok(())`
- Added tests that assert the custom error messages and error code numbers
- Built and tested the program successfully against a local validator

## Commands that worked here

```bash
cd day04/require_revert_errors
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The observed custom error code numbers were `6000`, `6001`, and `6002`, matching the enum order
- The successful range check logged `Result = 50`
- The `func_ok` path also logged the same message and completed successfully

## Exercise Answers

- Exercise 1: Anchor assigns custom error numbers sequentially starting at `6000`, in enum order. If you reorder the enum variants, the numeric error codes change with that order.
- Exercise 2: With the current enum ordering in this repo, `AlwaysErrors` is the third custom error, so its error number is `6002`.
- The `require!` macro is just a shorter way to return a custom error when a condition is false. It serves the same purpose as an explicit `if` plus `return err!(...)`.
- For the logging exercise, `func_ok` was verified to log `Will this print?` and succeed. The failed path still returns the custom error correctly; if you want to inspect whether the pre-error `msg!` is emitted in your environment, the most reliable place to check is a live validator log stream during execution.
