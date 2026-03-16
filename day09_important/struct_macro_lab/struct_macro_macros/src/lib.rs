extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn foo_bar_attribute(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;
    let visibility = &input.vis;

    TokenStream::from(quote! {
        #[derive(Debug)]
        #visibility struct #struct_name {
            foo: i32,
            bar: i32,
        }

        impl Default for #struct_name {
            fn default() -> Self {
                Self { foo: 10, bar: 20 }
            }
        }

        impl #struct_name {
            fn double_foo(&self) -> i32 {
                self.foo * 2
            }
        }
    })
}

#[proc_macro_attribute]
pub fn destroy_attribute(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;
    let visibility = &input.vis;

    TokenStream::from(quote! {
        #[derive(Debug)]
        #visibility struct #struct_name {}
    })
}

#[proc_macro_derive(DoubleFoo)]
pub fn derive_double_foo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let has_named_foo = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields.named.iter().any(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|ident| ident == "foo")
                    .unwrap_or(false)
            }),
            _ => false,
        },
        _ => false,
    };

    if !has_named_foo {
        return TokenStream::from(quote! {
            compile_error!("DoubleFoo can only be derived for structs with a named `foo` field.");
        });
    }

    TokenStream::from(quote! {
        impl #struct_name {
            fn double_foo(&self) -> i32 {
                self.foo * 2
            }
        }
    })
}
