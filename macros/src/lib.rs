use authenticated::auth_function::simple_type_check_is_box_dyn_auth_token;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ItemFn, Stmt};

mod authenticated;

#[proc_macro_attribute]
pub fn authenticated(_: TokenStream, input: TokenStream) -> TokenStream {
    println!("Do transform 'authenticate'");
    let temp_input = input.clone();
    let mut ast = parse_macro_input!(input as ItemFn);

    let func_args = ast.sig.inputs.clone();

    let mut box_dyn_auth_token_param = None;
    let mut others = vec![];

    for param in func_args.iter() {
        if let FnArg::Typed(param_type) = param {
            if simple_type_check_is_box_dyn_auth_token(&*param_type.ty) {
                box_dyn_auth_token_param = Some(param);
                // ToDo: check if session already exists in param list !!
                others.push(syn::parse_quote!(session: types::Session));
            } else {
                others.push(param.clone());
            }
        }
    }

    match box_dyn_auth_token_param {
        Some(_) => {
            let func_args_modified: Punctuated<FnArg, Comma> = Punctuated::from_iter(others);
            ast.sig.inputs = func_args_modified;
            // TODO: get name from box_dyn_auth_token_param
            let token_init = quote! {
                let token: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
            };

            let first_stmt: Stmt = syn::parse2(token_init).expect("Could not parse token init");
            let mut with_init_stmt = vec![];

            with_init_stmt.push(first_stmt);
            for stmt in ast.block.stmts {
                with_init_stmt.push(stmt);
            }

            ast.block.stmts = with_init_stmt;

            quote! {
                #ast
            }.into()
        },
        None => TokenStream::from(
            syn::Error::new_spanned(ast, "No 'Box<dyn AuthToken>' found in parameter list").to_compile_error()
        ),
    }

}

#[proc_macro_attribute]
pub fn show(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}


