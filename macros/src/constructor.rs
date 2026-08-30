use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => {
            return quote! {
                compile_error!("Constructor can only be derived on structs");
            }
            .into();
        }
    };

    match &data.fields {
        Fields::Named(fields) => {
            let field_names: Vec<_> = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect();

            let field_types: Vec<_> = fields.named.iter().map(|field| &field.ty).collect();

            quote! {
                impl #name {
                    pub fn new(
                        #(#field_names: #field_types),*
                    ) -> Self {
                        Self {
                            #(#field_names),*
                        }
                    }
                }
            }
            .into()
        }
        Fields::Unnamed(fields) => {
            let params: Vec<_> = (0..fields.unnamed.len())
                .map(|i| Ident::new(&format!("field_{i}"), Span::call_site()))
                .collect();

            let field_types = fields.unnamed.iter().map(|field| &field.ty);

            quote! {
                impl #name {
                    pub fn new(
                        #(#params: #field_types),*
                    ) -> Self {
                        Self(
                            #(#params),*
                        )
                    }
                }
            }
            .into()
        }
        Fields::Unit => quote! {
            impl #name {
                pub fn new() -> Self {
                    Self
                }
            }
        }
        .into(),
    }
}
