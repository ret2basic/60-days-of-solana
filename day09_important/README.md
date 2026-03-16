# Day 9

This day is complete in `day09_important/struct_macro_lab`.

## What was done

- Created a pure Rust workspace instead of an Anchor project because procedural macros must live in a dedicated `proc-macro` crate
- Implemented the article's `impl` and trait examples in `struct_macro_demo`
- Added two attribute-like macros in `struct_macro_macros`: `foo_bar_attribute` and `destroy_attribute`
- Added a custom derive macro `DoubleFoo` to show how derive macros augment a struct without rewriting it
- Added tests for struct methods, trait behavior, and macro-generated methods
- Added a separate compile-fail example for `destroy_attribute` to demonstrate that the macro removes fields from the struct

## Commands that worked here

```bash
cd day09_important/struct_macro_lab
cargo run -p struct_macro_demo
cargo test -p struct_macro_demo
cargo check -p struct_macro_demo --example destroy_attribute_fail --features compile-fail-demo
```

## Notes

- The procedural macros live in `struct_macro_macros/src/lib.rs`
- The runnable examples for `impl`, traits, attribute macros, and derive macros live in `struct_macro_demo/src/main.rs`
- The article's wording is slightly misleading here: `foo_bar_attribute` does not preserve the original fields and then append `foo` and `bar`. It fully rewrites the struct into one with only `foo` and `bar`, then provides `Default` and adds `double_foo()`
- `destroy_attribute` removes the struct fields entirely, so constructing the original field names fails at compile time
- `DoubleFoo` is a custom derive macro that adds a `double_foo()` method to any struct with a named `foo` field
- The important distinction is: derive macros usually augment an existing type, while attribute-like procedural macros can completely replace the item they are attached to

## Exercise Answers

- Exercise 1: The `impl` example was implemented with `Person::new`, `can_drink`, and `age_in_one_year` in `struct_macro_demo/src/main.rs`.
- Exercise 2: The trait example was implemented with `Speed` plus `Car` and `Boat`, converting miles per hour and knots into kilometers per hour.
- Exercise 3: The article describes this as "inserting fields," but the example macro actually replaces the original struct definition with a new one containing only `foo` and `bar`. That behavior was implemented as `foo_bar_attribute` in `struct_macro_macros/src/lib.rs`, and it also adds a `double_foo()` method.
- Exercise 4: The attribute-like macro that removes fields was implemented as `destroy_attribute` in `struct_macro_macros/src/lib.rs`. The compile-fail example is `struct_macro_demo/examples/destroy_attribute_fail.rs`.
- Exercise 5: The custom derive example was implemented as `DoubleFoo` in `struct_macro_macros/src/lib.rs`. Unlike the attribute macros, it augments an existing struct instead of replacing its fields.