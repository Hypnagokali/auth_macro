use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn show(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

#[proc_macro_attribute]
pub fn authenticated(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn  = input.clone();
    let ast = parse_macro_input!(input_fn as ItemFn);

    input
}