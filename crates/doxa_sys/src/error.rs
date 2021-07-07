use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

pub fn derive_respondable_error(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let variant_idents = match data {
        syn::Data::Enum(e) => e.variants,
        _ => panic!("You can only derive `RespondableError` on an enum"),
    }
    .into_iter()
    .map(|variant| {
        let ident = variant.ident;
        match variant.fields {
            Fields::Unnamed(fields) => {
                assert_eq!(
                    fields.unnamed.len(),
                    1,
                    "Only 1 element is allowed in any field of the enum"
                );
            }
            _ => panic!("Only unnamed fields are supported"),
        };

        ident
    })
    .collect::<Vec<_>>();

    let output = quote! {
        impl crate::error::RespondableError for #ident {
            fn error_code(&self) -> String {
                match self {
                    #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::error_code(e)),*
                }
            }

            fn error_message(&self) -> Option<String> {
                match self {
                    #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::error_message(e)),*
                }
            }

            fn status_code(&self) -> actix_web::http::StatusCode {
                match self {
                    #(#ident::#variant_idents(e) => doxa_core::error::RespondableError::status_code(e)),*
                }
            }
        }
    };

    output.into()
}
