# Day 10

This day is complete in `day10/visibility_lab`.

## What was done

- Created an Anchor project named `visibility_lab`
- Adapted the article into runnable visibility demos that keep the program buildable while still covering the article's main ideas
- Added one state account so tests can verify file-private helpers, internal module access, restricted visibility, and shared logic from both a separate file and an inline module
- Added TypeScript tests covering all 5 day10 behaviors against a local validator

## Commands that worked here

```bash
cd day10/visibility_lab
yarn install
anchor build
solana-test-validator --reset
solana config set --url localhost
solana airdrop 100 $(solana address)
anchor test --skip-local-validator
```

## Notes

- The deployed program id was `45XGNnwXwFGzpMDwPrDy7iy1NGSigV4GcT1nGwVqdCEH`
- `file_private_demo()` calls `get_a_num()` from outside the `#[program]` module, which matches the article's simple private-like helper example
- `internal_visibility_demo()` calls `some_internal_function::internal_function()` directly and also through `external_views::call_internal_from_outside_program_module()`, showing that a nested `pub` module is usable elsewhere in the crate without becoming an instruction in the IDL
- `restricted_visibility_demo()` uses `pub(in crate::visibility_lab)` so the helper is callable from the parent program module but not from unrelated modules
- The article's failing outside call for the private example is intentionally described rather than checked into buildable program code, because keeping that call active would prevent the day10 project from compiling with the expected `E0624` privacy error
- The shared-logic section is represented twice: `calculate.rs` is the separate-file version and `inline_calculate` is the same idea declared inline in `lib.rs`

## Exercise Answers

- Public versus external: every handler inside the `#[program]` module is `pub`, so the practical external surface for the program is the instruction set generated into the IDL.
- File-private helpers: `get_a_num()` shows the article's simplest private-like pattern. It is not `pub`, but program handlers in the same file can still call it.
- Internal visibility: `some_internal_function::internal_function()` is public inside a nested module, so both the handler and the separate `external_views` module can call it.
- Restricted visibility: `some_private_function::private_function()` uses `pub(in crate::visibility_lab)`, so it stays visible to the parent program module while remaining unavailable to unrelated modules.
- Modules as the inheritance analog: `calculate::add()` in `calculate.rs` and `inline_calculate::add()` in `lib.rs` both show the Rust way to share functionality instead of relying on Solidity-style inheritance.