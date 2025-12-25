use std::env::home_dir;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

use crate::entry::{
    app::AppAttributes,
    config_builder::{StructDef, generate_structs},
};

mod app;
mod config_builder;

pub(crate) fn config_builder(input: TokenStream) -> TokenStream {
    let mut struct_def = parse_macro_input!(input as StructDef);
    struct_def.attr =
        Some(parse_quote!(#[derive(zr_app::Config, serde::Serialize, serde::Deserialize)]));
    let generated = generate_structs(&struct_def);
    generated.into()
}

pub(crate) fn app(attr: TokenStream, input: TokenStream) -> TokenStream {
    let AppAttributes { conf, app_folder } = parse_macro_input!(attr as AppAttributes);
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(input as ItemFn);
    let app_folder = app_folder
        .unwrap_or(String::from("."))
        .replace("~", home_dir().unwrap().to_str().unwrap());
    let conf_file = format!("{}/config.conf", app_folder.clone());
    quote! {
        #(#attrs)*
        #vis #sig {
            let app_folder = #app_folder;
            std::fs::create_dir_all(#app_folder).unwrap();
            let config: #conf = zr_app::config::get_config(#conf_file);
            #block
        }
    }
    .into()
}
