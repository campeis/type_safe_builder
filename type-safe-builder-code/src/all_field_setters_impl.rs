use crate::parse::{Field, FromStruct};
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn create(from_struct: &FromStruct) -> Vec<TokenStream> {
    from_struct
        .fields
        .iter()
        .map(|field| setter_impl_for(field, from_struct))
        .collect()
}

fn setter_impl_for(field: &Field, from_struct: &FromStruct) -> TokenStream {
    let field_ident = field.ident();
    let field_type = field.ty();

    let other_placeholder_field_type_ident = from_struct.fields.iter().filter_map(|other_field| {
        if other_field.ident() == field.ident() {
            None
        } else {
            let field_placeholder = other_field.field_placeholder();
            Some(quote! {const #field_placeholder: bool})
        }
    });

    let input_placeholder_field_type_ident = from_struct.fields.iter().map(|other_field| {
        if other_field.ident() == field.ident() {
            quote! {false}
        } else {
            let field_placeholder = other_field.field_placeholder();
            quote! {#field_placeholder}
        }
    });

    let output_placeholder_field_type_ident = from_struct.fields.iter().map(|other_field| {
        if other_field.ident() == field.ident() {
            quote! {true}
        } else {
            let field_placeholder = other_field.field_placeholder();
            quote! {#field_placeholder}
        }
    });

    let copy_other_fields = from_struct.fields.iter().filter_map(|other_field| {
        if other_field.ident() == field.ident() {
            None
        } else {
            let other_field_ident = other_field.ident();
            Some(quote! {#other_field_ident: self.#other_field_ident})
        }
    });

    let generics = from_struct.generics.all();

    let all_generics_names1 = from_struct.generics.all_names();
    let all_generics_names2 = all_generics_names1.clone();

    let where_clause = from_struct.generics.where_clause();

    let builder_state_ident = from_struct.builder_state_ident();
    quote! {
        impl<#(#generics,)*#(#other_placeholder_field_type_ident,)*> #builder_state_ident<#(#all_generics_names1,)*#(#input_placeholder_field_type_ident,)*> #where_clause {
        fn #field_ident(self, value: #field_type) -> #builder_state_ident<#(#all_generics_names2,)*#(#output_placeholder_field_type_ident,)*> {
            #builder_state_ident {
                #field_ident: Some(value),
                #(#copy_other_fields,)*
                }
            }
        }
    }
}
