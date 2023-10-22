use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{GenericParam, Generics};

pub(super) fn create(
    fields: &[NamedField],
    builder_state_ident: &Ident,
    generics: &Generics,
) -> Vec<TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.name;
        let field_type = &field.ty;
        let field_ident = format_ident!("{}", field_name);

        let other_phantom_field_type_ident = fields.iter().filter_map(|field| {
            if field_name == &field.name {
                None
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.name);
                Some(quote!{#phantom_field_type_ident})
            }
        });

        let input_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.name {
                quote!{type_safe_builder_code::Unset}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.name);
                quote!{#phantom_field_type_ident}
            }
        });

        let output_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.name{
                quote!{type_safe_builder_code::Set}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.name);
                quote!{#phantom_field_type_ident}
            }
        });

        let copy_other_fields = fields.iter().filter_map(|field| {
            let other_field = &field.name;
            if field_name == other_field{
                None
            } else {
                Some(quote!{#other_field: self.#other_field})
            }
        });

        let all_phantom_to_default = fields.iter().map(|field| {
            let field_name = &field.name;
            let phantom_field_ident = format_ident!("phantom_{}", field_name);

            quote! {
            #phantom_field_ident: Default::default()
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
            impl<#(#all_generics,)*#(#other_phantom_field_type_ident,)*> #builder_state_ident<#(#all_generics2,)*#(#input_phantom_field_type_ident,)*> #where_clause {
            fn #field_ident(self, value: #field_type) -> #builder_state_ident<#(#all_generics3,)*#(#output_phantom_field_type_ident,)*> {
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
