use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("AeuzUEiotxFUu1EEhVK1fTF1GvLrcEmQHiJ65cQZAMCe");

const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod tryrust {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!(
            "Answer to the ultimate question: {}",
            MEANING_OF_LIFE_AND_EXISTENCE
        );
        Ok(())
    }

    pub fn age_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        if age >= 18 {
            msg!("You are 18 years old or above");
        } else {
            msg!("You are below 18 years old");
        }
        Ok(())
    }

    pub fn ternary_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        let result = if age % 2 == 0 { true } else { false };
        msg!("{}", result);
        Ok(())
    }

    pub fn match_checker(_ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => msg!("The age is 1"),
            2 | 3 => msg!("The age is either 2 or 3"),
            4..=6 => msg!("The age is between 4 and 6"),
            _ => msg!("The age is something else"),
        }
        Ok(())
    }

    pub fn loop_demo(_ctx: Context<Initialize>) -> Result<()> {
        for value in (0..10).step_by(2) {
            msg!("{}", value);
        }
        Ok(())
    }

    pub fn vector_demo(_ctx: Context<Initialize>) -> Result<()> {
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];
        let first_element = my_array[0];
        let third_element = my_array[2];

        let mut mutable_array: [u32; 3] = [100, 200, 300];
        mutable_array[1] = 250;

        let mut dynamic_array: Vec<u32> = Vec::new();
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        msg!(
            "Fixed array values: first = {}, third = {}, mutable second = {}",
            first_element,
            third_element,
            mutable_array[1]
        );
        msg!("Third element = {}", dynamic_array[2]);
        Ok(())
    }

    pub fn hashmap_demo(
        _ctx: Context<Initialize>,
        key: String,
        value: String,
    ) -> Result<()> {
        let mut my_map = HashMap::new();
        my_map.insert(key.clone(), value);

        if let Some(stored_value) = my_map.get(&key) {
            msg!("My name is {}", stored_value);
        }

        Ok(())
    }

    pub fn struct_demo(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        struct Person {
            my_name: String,
            my_age: u64,
        }

        let mut person1 = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);
        Ok(())
    }

    pub fn usize_demo(_ctx: Context<Initialize>) -> Result<()> {
        let dynamic_array: Vec<u32> = Vec::from([1, 2, 3, 4, 5, 6]);
        let len = dynamic_array.len();
        let another_var: u64 = 5;
        let len_plus_another_var = len as u64 + another_var;

        msg!("The result is {}", len_plus_another_var);
        Ok(())
    }

    pub fn filter_even_numbers(_ctx: Context<Initialize>, values: Vec<u64>) -> Result<()> {
        let mut even_numbers: Vec<u64> = Vec::new();

        for value in values {
            if value % 2 == 0 {
                even_numbers.push(value);
            }
        }

        msg!("Even numbers = {:?}", even_numbers);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
