use crate::parse::{DefaultToSet, FromStruct};
use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

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
    let from_struct_ident = from_struct.ident();
    quote! {
        impl <#(#all_generics,)*#(#all_default_placeholder_fields_types,)*> #builder_state_ident<#(#all_generics2,)*#(#all_not_default_set,)*> #where_clause{
            fn build(self) -> #from_struct_ident<#(#all_generics3,)*> {
                #from_struct_ident {
                    #(#copy_all_fields,)*
                }
            }
        }
    }
}
