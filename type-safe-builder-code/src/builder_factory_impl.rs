use crate::parse::FromStruct;
use proc_macro2::TokenStream;
use quote::quote;

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

    let all_generics = from_struct.generics.all();
    let all_generics_names = from_struct.generics.all_names();

    let where_clause = from_struct.generics.where_clause();

    let builder_factory_ident = from_struct.builder_ident();
    let builder_state_ident = from_struct.builder_state_ident();

    quote! {
        impl #builder_factory_ident {
            pub fn builder<#(#all_generics,)*>() -> #builder_state_ident<#(#all_generics_names,)*#(#all_unset,)*> #where_clause {
                #builder_state_ident {
                    #(#all_unset_fields,)*
                }
            }
        }
    }
}
