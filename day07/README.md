# Day 7

This day is complete in `day07/rust_syntax_lab`.

## What was done

- Created an Anchor project named `rust_syntax_lab`
- Adapted the article's Rust syntax examples into separate instructions that compile and run on-chain
- Added demonstrations for ownership and borrowing, copy types, mutable variables, generics, `Option` with `unwrap()` and dereferencing, and `Result` with the `?` operator
- Added TypeScript tests covering all 6 instructions and verified them against a local validator

## Commands that worked here

```bash
cd day07/rust_syntax_lab
yarn install
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `6U61dGuSvEt1NSQhNjYzSbAwTi5SZ4AB5XxFBK9izKcm`
- The ownership demo logged `owner = abc`, `borrowed = abc`, `updated message = hello world`, and `cloned snapshot = hello`
- The copy-type demo logged `first = 3` and `second = 3`, showing that integers are copied instead of moved
- The mutable-variable demo logged `counter = 1`
- The generics demo logged `MyValue { foo: 1 }`, `MyValue { foo: false }`, `MyPair { foo: 7, bar: true }`, and then accessed the generic fields directly
- The `Option` plus dereference demo logged `max value = 5`
- The `Result` plus `?` demo encoded and decoded a `Person` struct and logged `My name is "Alice", I am 27 years old.`

## Exercise Answers

- Exercise 1: Ownership and borrowing were demonstrated in `ownership_demo()`. Borrowing with `&message` let the code read the string without moving it, and `clone()` produced an independent snapshot before the original string was mutated.
- Exercise 2: Copy semantics were demonstrated in `copy_type_demo()`. Assigning `first` into `second` left both values usable because `u64` implements `Copy`.
- Exercise 3: Mutable state was demonstrated in `mut_demo()`. The variable had to be declared with `mut` before incrementing it.
- Exercise 4: Generics were demonstrated with `MyValue<T>` and `MyPair<T, U>` in `generics_demo()`. This kept one structure definition reusable across multiple concrete types.
- Exercise 5: `Option` and dereferencing were demonstrated in `option_and_deref_demo()`. `iter().max()` returns an `Option<&u64>`, `unwrap()` extracts the inner reference, and `*` converts that reference into a plain `u64`.
- Exercise 6: `Result` and the `?` operator were demonstrated in `encode_and_decode(name, age)`. The helper returns `Result<Person>`, and `?` propagates decode failures without manual `match` boilerplate.