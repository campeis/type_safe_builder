use crate::parse::{DefaultToSet, FromStruct};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn create(from_struct: &FromStruct) -> TokenStream {
    let all_not_default_set = from_struct.fields.iter().map(|field| {
        if field.has_default() {
            let field_placeholder = field.field_placeholder();
            quote! {
                #field_placeholder
            }
        } else {
            quote! {
                true
            }
        }
    });

    let all_default_placeholder_fields_types = from_struct.fields.iter().filter_map(|field| {
        if field.has_default() {
            let field_placeholder = field.field_placeholder();
            Some(quote! {const #field_placeholder : bool})
        } else {
            None
        }
    });

    let copy_all_fields = from_struct.fields.iter().map(|field| {
        let field_name = field.ident();
        match field.default_to_set() {
            None => quote! {
                #field_name: self.#field_name.unwrap()
            },
            Some(DefaultToSet::AsDefault) => quote! {
                #field_name: self.#field_name.unwrap_or_default()
            },
            Some(DefaultToSet::AsValue(value)) => quote! {
                #field_name: #value
            },
        }
    });

    let all_generics = from_struct.generics.all();
    let all_generics_names1 = from_struct.generics.all_names();
    let all_generics_names2 = all_generics_names1.clone();

    let where_clause = from_struct.generics.where_clause();

    let builder_state_ident = from_struct.builder_state_ident();
    let from_struct_ident = from_struct.ident();
    quote! {
        impl <#(#all_generics,)*#(#all_default_placeholder_fields_types,)*> #builder_state_ident<#(#all_generics_names1,)*#(#all_not_default_set,)*> #where_clause{
            fn build(self) -> #from_struct_ident<#(#all_generics_names2,)*> {
                #from_struct_ident {
                    #(#copy_all_fields,)*
                }
            }
        }
    }
}
