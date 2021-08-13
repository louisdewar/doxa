use proc_macro::{self, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};

struct GuardTuples {
    name: Ident,
    count: LitInt,
}

impl Parse for GuardTuples {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let count = input.parse()?;
        Ok(GuardTuples { name, count })
    }
}

pub fn guard_tuples(input: TokenStream) -> TokenStream {
    let GuardTuples { name, count } = parse_macro_input!(input as GuardTuples);
    let count: i32 = count.base10_parse().unwrap();

    todo!("{}{}", name, count);

    //  let output = quote! {
    //      impl crate::error::RespondableError for #name {
    //          fn error_code(&self) -> String {
    //              match self {
    //                  // #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::error_code(e)),*
    //              }
    //          }

    //          fn error_message(&self) -> Option<String> {
    //              match self {
    //                  // #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::error_message(e)),*
    //              }
    //          }

    //          fn status_code(&self) -> actix_web::http::StatusCode {
    //              match self {
    //                  // #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::status_code(e)),*
    //              }
    //          }
    //      }
    //  };

    //  output.into()
}
