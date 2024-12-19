use authenticated::auth_function::AuthFunction;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input,ItemFn};

mod authenticated;

#[proc_macro_attribute]
pub fn authenticated(_: TokenStream, input: TokenStream) -> TokenStream {
    println!("Do transform 'authenticate'");
    let ast = parse_macro_input!(input as ItemFn);

    match AuthFunction::new(ast.clone()) {
        Ok(func) => func.into_token_stream().into(),
        Err(e) => TokenStream::from(e.to_compile_error()),
    }
}

#[proc_macro_attribute]
pub fn show(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}


