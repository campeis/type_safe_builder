use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Attribute;

pub(crate) fn create(
    fields: &[NamedField],
    builder_state_ident: &Ident,
    name: &Ident,
) -> TokenStream {
    let all_not_default_set = fields.iter().map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        let field_type = &field.ty;
        if is_with_default(&field.attrs) {
            quote! {
                #phantom_field_type_ident
            }
        } else {
            quote! {
                #field_type
            }
        }
    });

    let all_default_phantom_fields_types = fields.iter().filter_map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        if is_with_default(&field.attrs) {
            Some(quote! {#phantom_field_type_ident})
        } else {
            None
        }
    });

    let copy_all_fields = fields.iter().map(|field| {
        let field_name = &field.name;
        if is_with_default(&field.attrs) {
            default_to_set(&field.attrs)
                .map(|t| {
                    quote! {
                        #field_name: #t
                    }
                })
                .unwrap_or(quote! {
                    #field_name: self.#field_name.unwrap_or_default()
                })
        } else {
            quote! {
                #field_name: self.#field_name.unwrap()
            }
        }
    });

    quote! {
        impl <#(#all_default_phantom_fields_types,)*> #builder_state_ident<#(#all_not_default_set,)*> {
            fn build(self) -> #name {
                #name {
                    #(#copy_all_fields,)*
                }
            }
        }
    }
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("Phantom{}Type", field_name)
}

fn is_with_default(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("build_default"))
}

fn default_to_set(attrs: &[Attribute]) -> Option<TokenStream> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("build_default"))
        .and_then(|attr| attr.parse_args::<TokenStream>().ok())
}
