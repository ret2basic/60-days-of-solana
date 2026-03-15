# Day 2

This day is complete in `day02/arithmetic_basics`.

## What was done

- Implemented argument passing for `u64`, `String`, and `Vec<u64>`
- Added integer math instructions for add, subtract, multiply, divide, and power
- Added float math instructions for cube root, square root, and base-10 logarithm
- Added checked subtraction with a custom underflow error
- Built and tested the program successfully against a local validator

## Commands that worked here

```bash
cd day02/arithmetic_basics
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The program logs confirm the expected examples from the article, including `You said "hello"`, `Your array [777, 888]`, and `Result = 1024`
- The checked underflow test intentionally fails on-chain with the custom `Underflow` error and is asserted in the test suite
- Float operations work, but they consume noticeably more compute units than simple integer operations
