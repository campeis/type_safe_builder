use crate::parse::FromStruct;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn create(from_struct: &FromStruct) -> TokenStream {
    let state_fields_declarations = from_struct.fields.iter().map(|field| {
        let field_ident = field.ident();
        let field_type = field.ty();
        quote! {
            #field_ident: Option<#field_type>
        }
    });

    let all_placeholder_fields_types = from_struct.fields.iter().map(|field| {
        let field_placeholder = field.field_placeholder();
        quote! {
            const #field_placeholder : bool
        }
    });

    let generic_params = from_struct.generics.params.clone();
    let all_generics = generic_params.iter().map(|param| {
        quote! {
            #param
        }
    });

    let where_clause = from_struct.generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    let builder_state_ident = from_struct.builder_state_ident();

    quote! {
        struct #builder_state_ident<#(#all_generics,)*#(#all_placeholder_fields_types,)*> #where_clause {
            #(#state_fields_declarations,)*
        }
    }
}
