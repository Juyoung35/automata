mod options;

use proc_macro::TokenStream;
use syn::DeriveInput;
use darling::FromDeriveInput;

#[proc_macro_derive(OpsDerive, attributes(ops_derive))]
pub fn derive_ops_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    // println!("{ast:?}");
    let opts = match options::Options::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };
    println!("{opts:?}");
    TokenStream::new()
}