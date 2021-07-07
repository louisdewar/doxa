mod error;
mod guard;

use proc_macro::TokenStream;

#[proc_macro_derive(RespondableError)]
pub fn derive_respondable_error(input: TokenStream) -> TokenStream {
    error::derive_respondable_error(input)
}

#[proc_macro]
pub fn guard_impls(input: TokenStream) -> TokenStream {
    guard::guard_tuples(input)
}
