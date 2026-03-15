# Day 6

This day is complete in `day06/tryrust`.

## What was done

- Created an Anchor project named `tryrust`
- Implemented separate instructions for the main Rust concepts from the article instead of overloading a single `initialize` function
- Added examples for `if/else`, ternary-style assignment, `match`, stepped `for` loops, fixed arrays, vectors, `HashMap`, structs, constants, and `usize` casting
- Implemented the exercise that filters even numbers from a `Vec<u64>` into a new vector and logs the result
- Added tests covering all 10 instructions and verified them against a local validator

## Commands that worked here

```bash
cd day06/tryrust
yarn install
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `AeuzUEiotxFUu1EEhVK1fTF1GvLrcEmQHiJ65cQZAMCe`
- The constant example logged `Answer to the ultimate question: 42`
- The `match` example logged `The age is between 4 and 6` for input `5`
- The loop example logged `0`, `2`, `4`, `6`, and `8`
- The vector example logged `Fixed array values: first = 10, third = 30, mutable second = 250` and `Third element = 30`
- The `HashMap` example is in-memory only and logged `My name is Bob`
- The struct example logged `Alice is 20 years old` and then `Bob is 18 years old`
- The `usize` casting example logged `The result is 11`
- The even-number exercise logged `Even numbers = [2, 4, 6]`

## Exercise Answers

- Exercise 1: Passing `Alice` and `20` into the struct example produced the expected logs: `Alice is 20 years old` followed by `Bob is 18 years old`.
- Exercise 2: The vector-filtering exercise was implemented as `filter_even_numbers(values: Vec<u64>)`. It loops through the input, pushes even values into a new vector, and logs the result.
- Exercise 3: For the tested input `[1, 2, 3, 4, 5, 6]`, the exercise logged `Even numbers = [2, 4, 6]`.
- Exercise 4: Rust does not have a direct ternary operator, but `if/else` can be used as an expression and assigned to a variable.
- Exercise 5: Solidity mappings and Solana `HashMap` are not equivalent in persistence. The `HashMap` example here only exists in memory during instruction execution and is not stored on-chain.