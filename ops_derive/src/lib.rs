mod consts;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(OpsDerive)]
pub fn derive_ops_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    println!("{ast:?}");
    TokenStream::new()
}