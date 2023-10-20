use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, TokenStream as TokenStream2};
use syn::parse_macro_input;

#[derive(Default)]
struct Tags {
    env_var: Option<Ident>,
    env_default: Option<Literal>,
}

fn init_value(field: &syn::Field) -> TokenStream2 {
    let mut tags = Tags::default();
    for attr in &field.attrs {
        match attr.path() {
            p if p.is_ident("env_var") => tags.env_var = Some(attr.parse_args().unwrap()),
            p if p.is_ident("env_default") => tags.env_default = Some(attr.parse_args().unwrap()),
            _ => panic!("Unknown attribute"),
        }
    }
    let name = field.ident.as_ref().unwrap();
    let typ = &field.ty;
    if let Some(env_var) = tags.env_var {
        if let Some(env_default) = tags.env_default {
            return quote::quote! {
                #name: <#typ as FromEnv>::from_env(&stringify!(#env_var), std::env::var(&stringify!(#env_var)).ok(), Some(#env_default.to_owned())),
            };
        }
        return quote::quote! {
            #name: <#typ as FromEnv>::from_env(&stringify!(#env_var), std::env::var(&stringify!(#env_var)).ok(), None),
        };
    }
    if let Some(env_default) = tags.env_default {
        return quote::quote! {
            #name: <#typ as FromEnv>::from_env(&stringify!(#name), std::env::var(&stringify!(#name).to_uppercase()).ok(), Some(#env_default.to_owned())),
        };
    }
    quote::quote! {
        #name: <#typ as FromEnv>::from_env(&stringify!(#name), std::env::var(&stringify!(#name).to_uppercase()).ok(), None),
    }
}

#[proc_macro_derive(FromEnvDerive, attributes(env_var, env_default))]
pub fn from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    if let syn::Data::Struct(data) = input.data {
        let mut values = Vec::new();
        for field in data.fields {
            values.push(init_value(&field));
        }
        return quote::quote! {
            impl #name {
                pub fn from_env() -> Self {
                    Self {
                        #(#values)*
                    }
                }
            }
        }
        .into();
    }
    panic!("Only structs are supported");
}
