#![feature(proc_macro_diagnostic)]
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod child;
mod field;
mod spawnable;
use self::spawnable::SpawnableDerive;

#[proc_macro_derive(Spawnable, attributes(child, bundle))]
pub fn derive_spawnable(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let spawnable =  SpawnableDerive::from(derive_input);
    let output = quote! { #spawnable };
    output.into()
}
