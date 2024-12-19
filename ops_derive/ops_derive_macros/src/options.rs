use darling::{
    ast::Data,
    util::{Ignored, SpannedValue},
    Error, FromDeriveInput, FromField, FromMeta, Result,
};
use syn::{Attribute, Generics, Ident, Path};

/// Collector for struct-level information about the type deriving `DefaultFromSerde`.
#[derive(FromDeriveInput, Debug)]
#[darling(
    attributes(ops_derive),
    supports(struct_any),
    forward_attrs(allow, cfg),
    allow_unknown_fields
)]
pub struct Options {
    ident: Ident,
    attrs: Vec<Attribute>,
    #[darling(default)]
    convert: Option<ConvertMeta>,
    #[darling(default)]
    ops_self: Option<OpsSelfMeta>,
    #[darling(default)]
    ops_with: Option<OpsWithMeta>,
}

#[derive(FromMeta, Debug)]
pub struct ConvertMeta {
    #[darlingmap = bit_to_types]
    target: Vec<Ident>,
    // from: String,
    // into: String,
}

use ops_derive::{bit_to_traits, bit_to_types};
#[derive(FromMeta, Debug)]
pub struct OpsSelfMeta {
    #[darling(map = bit_to_traits)]
    ops: Vec<Ident>,
}

#[derive(FromMeta, Debug)]
pub struct OpsWithMeta {
    #[darling(map = bit_to_traits)]
    ops: Vec<Ident>,
}