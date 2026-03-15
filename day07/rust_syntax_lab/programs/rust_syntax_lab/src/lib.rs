use anchor_lang::prelude::*;

declare_id!("6U61dGuSvEt1NSQhNjYzSbAwTi5SZ4AB5XxFBK9izKcm");

#[derive(Debug)]
struct MyValue<T: core::fmt::Debug> {
    foo: T,
}

#[derive(Debug)]
struct MyPair<T: core::fmt::Debug, U: core::fmt::Debug> {
    foo: T,
    bar: U,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
struct Person {
    name: String,
    age: u64,
}

#[program]
pub mod rust_syntax_lab {
    use super::*;

    pub fn ownership_demo(_ctx: Context<Initialize>) -> Result<()> {
        let owner = String::from("abc");
        let borrowed = &owner;

        msg!("owner = {}", owner);
        msg!("borrowed = {}", borrowed);

        let mut message = String::from("hello");
        let cloned_snapshot = message.clone();
        message = message + " world";

        msg!("updated message = {}", message);
        msg!("cloned snapshot = {}", cloned_snapshot);
        Ok(())
    }

    pub fn copy_type_demo(_ctx: Context<Initialize>) -> Result<()> {
        let first: u32 = 3;
        let second = first;

        msg!("first = {}", first);
        msg!("second = {}", second);
        Ok(())
    }

    pub fn mut_demo(_ctx: Context<Initialize>) -> Result<()> {
        let mut counter = 0;
        counter += 1;

        msg!("counter = {}", counter);
        Ok(())
    }

    pub fn generics_demo(_ctx: Context<Initialize>) -> Result<()> {
        let first_struct: MyValue<i32> = MyValue { foo: 1 };
        let second_struct: MyValue<bool> = MyValue { foo: false };
        let pair_struct: MyPair<i32, bool> = MyPair {
            foo: 7,
            bar: true,
        };

        msg!("first generic = {:?}", first_struct);
        msg!("second generic = {:?}", second_struct);
        msg!("pair generic = {:?}", pair_struct);
        msg!(
            "generic field values = {}, {}, {}",
            first_struct.foo,
            second_struct.foo,
            pair_struct.bar
        );
        msg!("pair foo = {}", pair_struct.foo);
        Ok(())
    }

    pub fn option_and_deref_demo(_ctx: Context<Initialize>) -> Result<()> {
        let values = Vec::from([1, 2, 3, 4, 5]);
        let max_value = *values.iter().max().unwrap();

        msg!("max value = {}", max_value);
        Ok(())
    }

    pub fn encode_and_decode(_ctx: Context<Initialize>) -> Result<()> {
        let init_person = Person {
            name: "Alice".to_string(),
            age: 27,
        };

        let mut encoded_data = Vec::new();
        init_person
            .serialize(&mut encoded_data)
            .map_err(|_| error!(Day7Error::EncodeFailed))?;

        let decoded_person = decode_person(&encoded_data)?;

        msg!(
            "My name is {:?}, I am {:?} years old.",
            decoded_person.name,
            decoded_person.age
        );

        Ok(())
    }
}

fn decode_person(encoded_data: &[u8]) -> Result<Person> {
    Person::try_from_slice(encoded_data).map_err(|_| error!(Day7Error::DecodeFailed))
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum Day7Error {
    #[msg("Failed to encode the person struct")]
    EncodeFailed,
    #[msg("Failed to decode the person struct")]
    DecodeFailed,
}
