use quote::ToTokens;
use syn::Type;

pub (crate) struct AuthFunction {

}

pub (crate) fn simple_type_check_is_box_dyn_auth_token(t: &Type) -> bool {
    // Just for testing.
    let as_string=  t.to_token_stream().to_string().replace(' ', "");
    
    as_string == "Box<dynAuthToken>"
}