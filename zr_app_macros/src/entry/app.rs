use quote::ToTokens;
use syn::{Expr, Ident, Meta, Token, parse::Parse};

const CONF: &str = "conf";
const APP_FOLDER: &str = "app_folder";

#[derive(Debug, Default)]
pub(super) struct AppAttributes {
    pub conf: Option<Ident>,
    pub app_folder: Option<String>,
}

fn expr_to_ident(expr: &Expr) -> Ident {
    match expr {
        Expr::Path(path) => path.path.segments.first().unwrap().ident.clone(),
        _ => panic!("Expression must be a path"),
    }
}

impl Parse for AppAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut app_attributes = AppAttributes::default();
        while !input.is_empty() {
            let meta = Meta::parse(input)?;
            match meta {
                Meta::NameValue(value) => {
                    if value.path.is_ident(CONF) {
                        app_attributes.conf = Some(expr_to_ident(&value.value));
                    } else if value.path.is_ident(APP_FOLDER) {
                        app_attributes.app_folder =
                            Some(value.value.to_token_stream().to_string().replace("\"", ""));
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(app_attributes)
    }
}
