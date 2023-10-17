use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Field;

pub(super) fn create(
    fields: &Punctuated<Field, Comma>,
    builder_state_ident: &Ident,
) -> Vec<TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident.clone().expect("unnamed fields are not supported");
        let field_type = &field.ty;
        let field_ident = format_ident!("{}", field_name);

        let other_phantom_field_type_ident = fields.iter().filter_map(|field| {
            if field_name == &field.ident.clone().expect("unnamed fields are not supported") {
                None
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().expect("unnamed fields are not supported"));
                Some(quote!{#phantom_field_type_ident})
            }
        });

        let input_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.ident.clone().expect("unnamed fields are not supported") {
                quote!{type_safe_builder_code::Unset}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().expect("unnamed fields are not supported"));
                quote!{#phantom_field_type_ident}
            }
        });

        let output_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.ident.clone().expect("unnamed fields are not supported") {
                quote!{#field_type}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().expect("unnamed fields are not supported"));
                quote!{#phantom_field_type_ident}
            }
        });

        let copy_other_fields = fields.iter().filter_map(|field| {
            let other_field = &field.ident.clone().expect("unnamed fields are not supported");
            if field_name == other_field{
                None
            } else {
                Some(quote!{#other_field: self.#other_field})
            }
        });

        let all_phantom_to_default = fields.iter().map(|field| {
            let field_name = &field.ident.clone().expect("unnamed fields are not supported");
            let phantom_field_ident = format_ident!("phantom_{}", field_name);

            quote! {
            #phantom_field_ident: Default::default()
        }
        });

        quote! {
            impl<#(#other_phantom_field_type_ident,)*> #builder_state_ident<#(#input_phantom_field_type_ident,)*> {
            fn #field_ident(self, value: #field_type) -> #builder_state_ident<#(#output_phantom_field_type_ident,)*> {
                #builder_state_ident {
                    #field_ident: Some(value),
                    #(#copy_other_fields,)*
                    #(#all_phantom_to_default,)*
                    }
                }
            }
        }
    }).collect()
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("Phantom{}Type", field_name)
}
