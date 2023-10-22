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
                Some(quote!{const #phantom_field_type_ident: bool})
            }
        });

        let input_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.name {
                quote!{false}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.name);
                quote!{#phantom_field_type_ident}
            }
        });

        let output_phantom_field_type_ident = fields.iter().map(|field| {
            if field_name == &field.name{
                quote!{true}
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
                    }
                }
            }
        }
    }).collect()
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("PHANTOM{}TYPE", field_name.to_string().to_uppercase())
}
