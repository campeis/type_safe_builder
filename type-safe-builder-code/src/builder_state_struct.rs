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

    let all_generics = from_struct.generics.all();

    let where_clause = from_struct.generics.where_clause();

    let builder_state_ident = from_struct.builder_state_ident();

    quote! {
        struct #builder_state_ident<#(#all_generics,)*#(#all_placeholder_fields_types,)*> #where_clause {
            #(#state_fields_declarations,)*
        }
    }
}
