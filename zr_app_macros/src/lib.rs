use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod entry;

#[proc_macro_derive(Config)]
pub fn config(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = syn::parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();
    quote! {
        impl #generic_impl #generic_ty zr_app::config::Config for #ident #where_clause {}
    }
    .into()
}

#[proc_macro]
pub fn config_builder(input: TokenStream) -> TokenStream {
    entry::config_builder(input.into())
}
