use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, Block, FnArg, ItemFn, Pat, PatType, Signature, Stmt,
    Type, Visibility,
};

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct BoxedAuthToken {
    name: Ident,
    var: FnArg,
}

impl BoxedAuthToken {
    pub(crate) fn new(ast: ItemFn, param: FnArg, param_type: PatType) -> syn::Result<Self> {
        if let Pat::Ident(param_ident) = &*param_type.pat {
            Ok(BoxedAuthToken {
                name: param_ident.ident.clone(),
                var: param,
            })
        } else {
            Err(syn::Error::new_spanned(
                ast,
                "Can not construct BoxedAuthToken from given function argument.",
            ))
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct AuthFunction {
    visibility: Visibility,
    fn_name: Ident,
    boxed_auth_token: BoxedAuthToken,
    other_params: Vec<FnArg>,
    orig_sig: Signature,
    orig_block: Box<Block>,
}

impl AuthFunction {
    pub(crate) fn new(ast_input: ItemFn) -> syn::Result<Self> {
        let ast = ast_input.clone();
        let mut box_dyn_auth_token_param = None;
        let mut others = vec![];

        for param in ast.sig.inputs.iter() {
            if let FnArg::Typed(param_type) = param {
                if simple_type_check_is_box_dyn_auth_token(&*param_type.ty) {
                    box_dyn_auth_token_param =
                        match BoxedAuthToken::new(ast.clone(), param.clone(), param_type.clone()) {
                            Ok(boxed_token) => Some(boxed_token),
                            Err(e) => {
                                return Err(e);
                            }
                        };

                    // ToDo: check if session already exists in param list !!
                    others.push(syn::parse_quote!(session: types::Session));
                } else {
                    others.push(param.clone());
                }
            }
        }

        match box_dyn_auth_token_param {
            Some(boxed_auth_token) => Ok(AuthFunction {
                visibility: ast.vis,
                fn_name: ast.sig.ident.clone(),
                boxed_auth_token,
                other_params: others,
                orig_sig: ast.sig,
                orig_block: ast.block,
            }),
            None => Err(syn::Error::new_spanned(
                ast,
                "No 'Box<dyn AuthToken>' found in parameter list",
            )),
        }
    }
}

impl ToTokens for AuthFunction {
    fn to_tokens(&self, output: &mut TokenStream) {
        let Self {
            visibility,
            fn_name: _,
            boxed_auth_token,
            other_params,
            mut orig_sig,
            mut orig_block,
        } = self.clone();

        // get new parameter list
        let mut param_list: Punctuated<FnArg, Comma> = Punctuated::new();
        for arg in other_params {
            param_list.push(arg.clone());
        }
        orig_sig.inputs = param_list;

        // pulling arg Box<dyn AuthToken> into body as assignment
        // TODO: if session exists, it could have another name. use that name instead. And generate a more complex
        let var_name = boxed_auth_token.name;
        let init_stmt: Stmt = syn::parse2(quote! {
            let #var_name: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
        })
        .expect("Could not parse init token statement: Box<dyn Auth> = Box::new(SessionAuthToken::new(session))");

        orig_block = add_statement_to_block(init_stmt, orig_block);

        let stream = quote! {
            #visibility #orig_sig
            #orig_block
        };

        output.extend(stream)
    }
}

fn add_statement_to_block(stmt: Stmt, mut block: Box<Block>) -> Box<Block> {
    let mut stmts_with_init_auth_token = Vec::new();
    stmts_with_init_auth_token.push(stmt);
    for stmt in block.stmts.iter() {
        stmts_with_init_auth_token.push(stmt.clone());
    }
    block.stmts = stmts_with_init_auth_token;

    block
}

fn simple_type_check_is_box_dyn_auth_token(t: &Type) -> bool {
    // Just for testing.
    let as_string = t.to_token_stream().to_string().replace(' ', "");

    as_string == "Box<dynAuthToken>"
}
