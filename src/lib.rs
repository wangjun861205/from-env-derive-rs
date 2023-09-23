use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::parse_macro_input;

pub use from_env::FromEnv;

fn init_value(field: &syn::Field) -> TokenStream2 {
    for attr in &field.attrs {
        if attr.path().is_ident("env_var") {
            let name: &Ident = field.ident.as_ref().unwrap();
            let attr: Ident = attr.parse_args().unwrap();
            let typ = &field.ty;
            return quote::quote! {
                #name: <#typ as FromEnv>::from_env(&stringify!(#attr).to_uppercase()),
            };
        }
    }
    let name = field.ident.as_ref().unwrap();
    let typ = &field.ty;
    quote::quote! {
        #name: <#typ as FromEnv>::from_env(&stringify!(#name).to_uppercase()),
    }
}

#[proc_macro_derive(FromEnvDerive, attributes(env_var))]
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
                fn from_env() -> Self {
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
