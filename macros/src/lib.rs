use std::any::Any;

use authenticated::auth_function::{simple_type_check_is_box_dyn_auth_token, AuthFunction};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ItemFn, Stmt};

mod authenticated;

#[proc_macro_attribute]
pub fn authenticated(_: TokenStream, input: TokenStream) -> TokenStream {
    println!("Do transform 'authenticate'");
    let mut ast = parse_macro_input!(input as ItemFn);


    match AuthFunction::new(ast.clone()) {
        Ok(func) => func.into_token_stream().into(),
        Err(e) => TokenStream::from(e.to_compile_error()),
    }

    // println!("{}", t.to_string());

    // quote! {
    //     #ast
    // }.into()
}

#[proc_macro_attribute]
pub fn show(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}


