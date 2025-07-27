use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote};

use crate::entry::config_builder::{StructDef, generate_structs};

mod config_builder;

pub(crate) fn config_builder(input: TokenStream) -> TokenStream {
    let mut struct_def = parse_macro_input!(input as StructDef);
    struct_def.attr = Some(
        parse_quote!(#[derive(zr_app::Config, Default, serde::Serialize, serde::Deserialize)]),
    );
    let generated = generate_structs(&struct_def);
    generated.into()
}
