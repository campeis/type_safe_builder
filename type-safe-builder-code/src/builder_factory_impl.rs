use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Generics;

pub(super) fn create(
    fields: &[NamedField],
    builder_factory_ident: &Ident,
    builder_state_ident: &Ident,
    generics: &Generics,
) -> TokenStream {
    let all_unset_fields = fields.iter().map(|field| {
        let field_ident = &field.name;
        let phantom_field_ident = format_ident!("phantom_{}", field_ident.to_string());

        quote! {
            #field_ident: None,
            #phantom_field_ident: Default::default()
        }
    });

    let all_unset = fields.iter().map(|_| {
        quote! {
            type_safe_builder_code::Unset
        }
    });

    let generic_params = generics.params.clone();
    let all_generics = generic_params.iter().map(|param| {
        quote! {
            #param
        }
    });

    let all_generics2 = all_generics.clone();
    let all_generics3 = all_generics.clone();

    let where_clause = generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    quote! {
        impl <#(#all_generics,)*> #builder_factory_ident<#(#all_generics2,)*> #where_clause {
            pub fn builder() -> #builder_state_ident<#(#all_generics3,)*#(#all_unset,)*> {
                #builder_state_ident {
                    #(#all_unset_fields,)*
                }
            }
        }
    }
}
