use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Lit, Token, parse::Parse, parse_quote, token::Brace};

#[derive(Clone)]
pub(super) enum FieldType {
    Simple((Ident, Option<Lit>)),
    Nested {
        struct_name: Ident,
        fields: Vec<Field>,
    },
}

#[derive(Clone)]
pub(super) struct Field {
    name: Ident,
    field_type: FieldType,
}

pub(super) struct StructDef {
    pub(super) attr: Option<Attribute>,
    name: Ident,
    fields: Vec<Field>,
}

impl Parse for FieldType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            let content;
            syn::braced!(content in input);
            let mut fields = Vec::new();
            while !content.is_empty() {
                let field = content.parse::<Field>()?;

                if !content.is_empty() {
                    content.parse::<Token![,]>()?;
                };

                fields.push(field);
            }

            return Ok(FieldType::Nested {
                struct_name: ident,
                fields,
            });
        }

        let default = if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Some(input.parse::<Lit>()?)
        } else {
            None
        };
        Ok(FieldType::Simple((ident, default)))
    }
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let field_type: FieldType = input.parse::<FieldType>()?;
        Ok(Field { name, field_type })
    }
}

impl Parse for StructDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let mut fields = Vec::new();

        while !content.is_empty() {
            let field: Field = content.parse()?;
            fields.push(field);
            if !content.is_empty() {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(StructDef {
            attr: Some(parse_quote!(#[derive(Default, serde::Serialize, serde::Deserialize)])),
            name,
            fields,
        })
    }
}

pub(super) fn generate_structs(struct_def: &StructDef) -> TokenStream {
    let mut generated = TokenStream::new();
    let mut nested_structs = Vec::new();

    collect_nested_structs(&struct_def.fields, &mut nested_structs);

    for nested in &nested_structs {
        let nested_tokens = generate_struct_tokens(nested);
        generated.extend(nested_tokens);
    }

    let main_struct = generate_struct_tokens(struct_def);
    generated.extend(main_struct);
    generated
}

fn collect_nested_structs(fields: &[Field], nested_structs: &mut Vec<StructDef>) {
    for field in fields {
        if let FieldType::Nested {
            struct_name,
            fields: nested_fields,
        } = &field.field_type
        {
            let nested_struct = StructDef {
                attr: Some(parse_quote!(#[derive(serde::Serialize, serde::Deserialize)])),
                name: struct_name.clone(),
                fields: nested_fields.clone(),
            };
            collect_nested_structs(nested_fields, nested_structs);
            nested_structs.push(nested_struct);
        }
    }
}

fn generate_struct_tokens(struct_def: &StructDef) -> TokenStream {
    let struct_name = &struct_def.name;
    let attr = &struct_def.attr;
    let mut field_tokens = Vec::new();
    let mut default_tokens = Vec::new();
    for field in &struct_def.fields {
        let field_name = &field.name;

        match &field.field_type {
            FieldType::Simple((ty, default)) => {
                field_tokens.push(quote! {
                    pub #field_name: #ty
                });
                default_tokens.push(if let Some(lit) = default {
                    quote! { #field_name: #lit.into() }
                } else {
                    quote! { #field_name: std::default::Default::default() }
                });
            }
            FieldType::Nested { struct_name, .. } => {
                field_tokens.push(quote! {
                    pub #field_name: #struct_name
                });
                default_tokens.push(quote! {
                    #field_name: std::default::Default::default()
                });
            }
        }
    }

    quote! {
        #attr
        pub struct #struct_name {
            #(#field_tokens),*
        }

        impl std::default::Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#default_tokens),*
                }
            }
        }
    }
}
