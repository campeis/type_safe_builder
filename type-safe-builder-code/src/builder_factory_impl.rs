use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub(super) fn create(
    fields: &[NamedField],
    builder_factory_ident: &Ident,
    builder_state_ident: &Ident,
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

    quote! {
        impl #builder_factory_ident {
            pub fn builder() -> #builder_state_ident<#(#all_unset,)*> {
                #builder_state_ident {
                    #(#all_unset_fields,)*
                }
            }
        }
    }
}
