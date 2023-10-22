use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Generics;

pub(super) fn create(
    fields: &[NamedField],
    builder_state_ident: &Ident,
    generics: &Generics,
) -> TokenStream {
    let state_fields_declarations = fields.iter().map(|field| {
        let field_ident = &field.name;
        let field_type = &field.ty;
        quote! {
            #field_ident: Option<#field_type>
        }
    });

    let all_phantom_fields_types = fields.iter().map(|field| {
        let field_name = &field.name;
        let phantom_field_type_ident = phantom_field_type_ident(field_name);
        quote! {
            const #phantom_field_type_ident : bool
        }
    });

    let generic_params = generics.params.clone();
    let all_generics = generic_params.iter().map(|param| {
        quote! {
            #param
        }
    });

    let where_clause = generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    quote! {
        struct #builder_state_ident<#(#all_generics,)*#(#all_phantom_fields_types,)*> #where_clause {
            #(#state_fields_declarations,)*
        }
    }
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("PHANTOM{}TYPE", field_name.to_string().to_uppercase())
}
