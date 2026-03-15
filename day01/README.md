# Day 1

This day is complete in `day01/hello_world`.

## What was done

- Created an Anchor program named `day_1`
- Added `msg!("Hello, world!")` to the `initialize` instruction
- Built the program successfully
- Ran `anchor test --skip-local-validator` successfully against a local validator
- Verified the generated program log contains `Hello, world!`

## Commands that worked here

```bash
cd day01/hello_world
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The system Solana CLI is `2.3.4`
- The AVM-managed Anchor `0.32.1` was reinstalled from source to match the system `glibc 2.35`
- Earlier prebuilt AVM binaries expected a newer `glibc`, which is why plain `anchor` initially failed

## Exercise Answers

- If you rename `initialize` to `initialize2` in both the Rust program and the test, the test still works. The function name itself is not special.
- The important requirement is that the generated IDL and the test stay in sync, so both sides must use the same instruction name.
- After the test succeeds, the program log still shows the emitted `Hello, world!` message for the renamed instruction.
