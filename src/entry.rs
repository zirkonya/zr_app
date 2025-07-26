use std::fmt::Display;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Signature, Visibility, braced, parse::Parse, token};

struct ItemFn {
    outer_attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    brace_token: token::Brace,
    inner_attrs: Vec<Attribute>,
    stmts: Vec<proc_macro2::TokenStream>,
}

impl Parse for ItemFn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let outer_attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let inner_attrs = Attribute::parse_inner(&content)?;
        let mut buf = TokenStream::new();
        let mut stmts = Vec::new();
        while !content.is_empty() {
            if let Some(semi) = content.parse::<Option<syn::Token![;]>>()? {
                semi.to_tokens(&mut buf);
                stmts.push(buf);
                buf = TokenStream::new();
                continue;
            }
            buf.extend([content.parse::<TokenStream>()?]);
        }
        if !buf.is_empty() {
            stmts.push(buf);
        }
        Ok(Self {
            outer_attrs,
            vis,
            sig,
            brace_token,
            inner_attrs,
            stmts,
        })
    }
}

impl Display for ItemFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {{\n {} \n}}",
            self.vis.to_token_stream(),
            self.sig.to_token_stream(),
            self.stmts
                .iter()
                .map(|line| line.to_token_stream().to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub(crate) fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let Ok(main) = syn::parse2::<ItemFn>(input.clone()) else {
        panic!("zr_app::main must be used on main function");
    };
    let vis = main.vis;
    let sig = main.sig;
    let stmts = main.stmts;
    quote::quote! {
        #vis #sig {
            #(#stmts);*
        }
    }
}
