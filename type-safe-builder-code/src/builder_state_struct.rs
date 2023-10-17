use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Field;

pub(super) fn create(
    fields: &Punctuated<Field, Comma>,
    builder_state_ident: &Ident,
) -> TokenStream {
    let state_fields_declarations = fields.iter().map(|field| {
        let field_name = &field
            .ident
            .clone()
            .expect("unnamed fields are not supported");
        let field_type = &field.ty;
        let field_ident = format_ident!("{}", field_name);
        let phantom_field_ident = format_ident!("phantom_{}", field_name);
        let phantom_field_type_ident = format_ident!("Phantom{}Type", field_name);
        quote! {
            #field_ident: Option<#field_type>,
            #phantom_field_ident: core::marker::PhantomData<#phantom_field_type_ident>
        }
    });

    let all_phantom_fields_types = fields.iter().map(|field| {
        let field_name = &field
            .ident
            .clone()
            .expect("unnamed fields are not supported");
        let phantom_field_type_ident = phantom_field_type_ident(field_name);
        quote! {
            #phantom_field_type_ident
        }
    });

    quote! {
        struct #builder_state_ident<#(#all_phantom_fields_types,)*> {
            #(#state_fields_declarations,)*
        }
    }
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("Phantom{}Type", field_name)
}
