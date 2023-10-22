use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, GenericParam, Generics};

pub(crate) fn create(
    fields: &[NamedField],
    builder_state_ident: &Ident,
    generics: &Generics,
    name: &Ident,
) -> TokenStream {
    let all_not_default_set = fields.iter().map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        if is_with_default(&field.attrs) {
            quote! {
                #phantom_field_type_ident
            }
        } else {
            quote! {
                type_safe_builder_code::Set
            }
        }
    });

    let all_default_phantom_fields_types = fields.iter().filter_map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        if is_with_default(&field.attrs) {
            Some(quote! {#phantom_field_type_ident})
        } else {
            None
        }
    });

    let copy_all_fields = fields.iter().map(|field| {
        let field_name = &field.name;
        if is_with_default(&field.attrs) {
            default_to_set(&field.attrs)
                .map(|t| {
                    quote! {
                        #field_name: #t
                    }
                })
                .unwrap_or(quote! {
                    #field_name: self.#field_name.unwrap_or_default()
                })
        } else {
            quote! {
                #field_name: self.#field_name.unwrap()
            }
        }
    });

    let generic_params = generics.params.clone();
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

    let where_clause = generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    quote! {
        impl <#(#all_generics,)*#(#all_default_phantom_fields_types,)*> #builder_state_ident<#(#all_generics2,)*#(#all_not_default_set,)*> #where_clause{
            fn build(self) -> #name<#(#all_generics3,)*> {
                #name {
                    #(#copy_all_fields,)*
                }
            }
        }
    }
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("Phantom{}Type", field_name)
}

fn is_with_default(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("build_default"))
}

fn default_to_set(attrs: &[Attribute]) -> Option<TokenStream> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("build_default"))
        .and_then(|attr| attr.parse_args::<TokenStream>().ok())
}
