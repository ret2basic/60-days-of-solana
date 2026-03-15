# Day 8

This day is complete in `day08/macro_lab`.

## What was done

- Created an Anchor project named `macro_lab`
- Adapted the article into a small lab that contrasts function-like macros with regular fixed-arity helper functions
- Added a runtime example for `msg!` and `format!`, a manual `write_one` / `write_two` / `write_three` comparison, and a short inventory of Anchor's macro categories
- Added TypeScript tests covering all 3 instructions and verified them against a local validator

## Commands that worked here

```bash
cd day08/macro_lab
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `6cWAZo6vX3aNxRkWAfq3fvJyjQj1NNDraXeAQuWx7562`
- The function-like macro demo logged `macro accepts one value: 1`, `macro accepts many values: 1 2 3`, and `format! also expands code: 2 + 3 = 5`
- The fixed-arity helper demo logged `one arg function = Hello, world!`, `two arg function = Hello macro`, and `three arg function = Rust macros expand`
- The Anchor macro inventory demo logged `#[program] is an attribute-like macro`, `#[derive(Accounts)] is a custom derive macro`, and `msg! is a function-like macro`
- The main lesson is that a regular Rust function has a fixed signature, while a macro can expand into the exact Rust code needed for the invocation site

## Exercise Answers

- Exercise 1: The difference between a function and a function-like macro was demonstrated by `function_like_macro_demo()` versus `fixed_arity_function_demo()`. `msg!` accepted multiple formatted values in one invocation, while the regular helper approach needed separate `write_one`, `write_two`, and `write_three` functions.
- Exercise 2: `msg!` uses `!` because it is not a normal function call. It is a function-like macro that expands into Rust code during compilation.
- Exercise 3: `format!` is another function-like macro. In this lab it built the string `2 + 3 = 5` before that string was logged with `msg!`.
- Exercise 4: `#[program]` is an attribute-like macro, `#[derive(Accounts)]` is a custom derive macro, and `msg!` is a function-like macro. The day08 program explicitly logged those categories in `anchor_macro_inventory()`.
- Exercise 5: The manual `write_one`, `write_two`, and `write_three` helpers show why macros are useful. If Rust relied only on regular functions for this pattern, each argument count would need its own implementation.