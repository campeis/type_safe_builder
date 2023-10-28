use crate::parse::{Field, FromStruct};
use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

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

    let generic_params = from_struct.generics.params.clone();
    let all_generics = generic_params.iter().map(|param| {
        quote! {
            #param
        }
    });

    let all_generics2 = generic_params.iter().map(|gen| match gen {
        GenericParam::Lifetime(l) => {
            let l = &l.lifetime;
            quote! {#l}
        }
        GenericParam::Type(t) => {
            let i = &t.ident;
            quote! {#i}
        }
        GenericParam::Const(c) => {
            let i = &c.ident;
            quote! {#i}
        }
    });
    let all_generics3 = all_generics2.clone();

    let where_clause = from_struct.generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    let builder_state_ident = from_struct.builder_state_ident();
    quote! {
        impl<#(#all_generics,)*#(#other_placeholder_field_type_ident,)*> #builder_state_ident<#(#all_generics2,)*#(#input_placeholder_field_type_ident,)*> #where_clause {
        fn #field_ident(self, value: #field_type) -> #builder_state_ident<#(#all_generics3,)*#(#output_placeholder_field_type_ident,)*> {
            #builder_state_ident {
                #field_ident: Some(value),
                #(#copy_other_fields,)*
                }
            }
        }
    }
}
