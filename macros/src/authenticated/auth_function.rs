use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Block, FnArg, ItemFn, Pat, PatType, Signature, Stmt, Type, Visibility};


pub (crate) struct BoxedAuthToken {
    name: Ident,
    var: FnArg,
}

impl BoxedAuthToken {
    pub (crate) fn new(ast: ItemFn, param: FnArg, param_type: PatType) -> syn::Result<Self> {
        if let Pat::Ident(param_ident) = &*param_type.pat {
            Ok(
                BoxedAuthToken {
                    name: param_ident.ident.clone(),
                    var: param,
                }
            )
        } else {
            Err(syn::Error::new_spanned(ast, "Can not construct BoxedAuthToken from given function argument."))
        }

    }
}

pub (crate) struct AuthFunction {
    visibility: Visibility,
    fn_name: Ident, 
    boxed_auth_token: BoxedAuthToken,
    other_params: Vec<FnArg>,
    orig_sig: Signature,
    orig_block: Box<Block>
}

impl AuthFunction {
    pub (crate) fn new(ast_input: ItemFn) -> syn::Result<Self> {
        let mut ast = ast_input.clone();

        let func_args = ast.sig.inputs.clone();
        
        let mut box_dyn_auth_token_param = None;
        let mut others = vec![];

        for param in func_args.iter() {
            if let FnArg::Typed(param_type) = param {
                if simple_type_check_is_box_dyn_auth_token(&*param_type.ty) {
                    box_dyn_auth_token_param = match BoxedAuthToken::new(ast.clone(), (*param).clone(),(*param_type).clone()) {
                        Ok(boxed_token) => Some(boxed_token),
                        Err(e) => {
                            return Err(e);
                        },
                    };

                    // ToDo: check if session already exists in param list !!
                    others.push(syn::parse_quote!(session: types::Session));
                } else {
                    others.push(param.clone());
                }
            }
        };
    
        match box_dyn_auth_token_param {
            Some(boxed_auth_token) => {
                Ok(AuthFunction {
                    visibility: ast.vis,
                    fn_name: ast.sig.ident.clone(),
                    boxed_auth_token,
                    other_params: others,
                    orig_sig: ast.sig,
                    orig_block: ast.block,
                })
    
            
            },
            None => Err(syn::Error::new_spanned(ast, "No 'Box<dyn AuthToken>' found in parameter list"))
        }
    }
}


impl ToTokens for AuthFunction {
    fn to_tokens(&self, output: &mut TokenStream) {
        let Self {
            visibility,
            fn_name,
            boxed_auth_token,
            other_params,
            orig_sig,
            orig_block,
        } = self;

        let mut param_list: Punctuated<FnArg, Comma> = Punctuated::new();
        for arg in other_params {
            param_list.push(arg.clone());
        }

        let mut new_sig = orig_sig.clone();
        new_sig.inputs = param_list;
        let new_var = boxed_auth_token.var.clone();

        let var_name = boxed_auth_token.name.clone();
        let boxed_auth_token_token_stream = quote! {
            let #var_name: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
        };
        let init_stmt: Stmt = syn::parse2(boxed_auth_token_token_stream).expect("Could not parse init token statement");
        let mut new_stmts = Vec::new();
        new_stmts.push(init_stmt);
        for stmt in orig_block.stmts.iter() {
            new_stmts.push(stmt.clone());
        }
        let mut new_block = orig_block.clone();
        new_block.stmts = new_stmts;

        let stream = quote! {
            #new_sig
            #new_block
        };

        output.extend(stream)
    }
}

pub (crate) fn simple_type_check_is_box_dyn_auth_token(t: &Type) -> bool {
    // Just for testing.
    let as_string=  t.to_token_stream().to_string().replace(' ', "");
    
    as_string == "Box<dynAuthToken>"
}