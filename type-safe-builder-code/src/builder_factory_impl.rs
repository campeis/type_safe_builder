use crate::parse::FromStruct;
use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

pub(super) fn create(from_struct: &FromStruct) -> TokenStream {
    let all_unset_fields = from_struct.fields.iter().map(|field| {
        let field_ident = &field.ident();

        quote! {
            #field_ident: None
        }
    });

    let all_unset = from_struct.fields.iter().map(|_| {
        quote! {
            false
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

    let where_clause = from_struct.generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    let builder_factory_ident = from_struct.builder_ident();
    let builder_state_ident = from_struct.builder_state_ident();

    quote! {
        impl #builder_factory_ident {
            pub fn builder<#(#all_generics,)*>() -> #builder_state_ident<#(#all_generics2,)*#(#all_unset,)*> #where_clause {
                #builder_state_ident {
                    #(#all_unset_fields,)*
                }
            }
        }
    }
}
