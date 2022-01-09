mod error;

use proc_macro::TokenStream;

#[proc_macro_derive(RespondableError)]
pub fn derive_respondable_error(input: TokenStream) -> TokenStream {
    error::derive_respondable_error(input)
}
