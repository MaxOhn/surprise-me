use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident};

use self::surprise::impl_surprise;

mod attributes;
mod factor;
mod match_arms;
mod surprise;
mod util;

/// Derive macro for the `Surprise` trait.
///
/// Check the trait's description for more information.
#[proc_macro_derive(Surprise, attributes(factor, weight))]
pub fn surprise(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    match impl_surprise(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => dummy_surprise(err, name).into(),
    }
}

fn dummy_surprise(err: Error, name: Ident) -> TokenStream2 {
    let err = err.to_compile_error();

    quote! {
        #err

        impl Surprise for #name {
            type Factor = ::surprise_me::factors::UnitSurprise;
        }

        impl ::surprise_me::Distribution<#name> for ::surprise_me::factors::UnitSurprise {
            fn sample<R: ::surprise_me::rand::Rng + ?Sized>(&self, rng: &mut R) -> #name {
                unimplemented!()
            }
        }
    }
}
