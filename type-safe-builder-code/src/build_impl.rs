use crate::NamedField;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{Attribute, GenericParam, Generics, Meta, Token};

pub(crate) fn create(
    fields: &[NamedField],
    builder_state_ident: &Ident,
    generics: &Generics,
    name: &Ident,
) -> TokenStream {
    let all_not_default_set = fields.iter().map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        if default_to_set(&field.attrs).is_some() {
            quote! {
                #phantom_field_type_ident
            }
        } else {
            quote! {
                true
            }
        }
    });

    let all_default_phantom_fields_types = fields.iter().filter_map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.name);
        if default_to_set(&field.attrs).is_some() {
            Some(quote! {const #phantom_field_type_ident : bool})
        } else {
            None
        }
    });

    let copy_all_fields = fields.iter().map(|field| {
        let field_name = &field.name;
        match default_to_set(&field.attrs) {
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
    format_ident!("PHANTOM{}TYPE", field_name.to_string().to_uppercase())
}

enum DefaultToSet {
    AsDefault,
    AsValue(TokenStream),
}
fn default_to_set(attrs: &[Attribute]) -> Option<DefaultToSet> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
        .and_then(|attr| {
            let values: Result<Punctuated<Meta, Token![,]>, _> =
                attr.parse_args_with(Punctuated::parse_terminated);

            match values {
                Ok(values) => values.iter().find_map(|m| match m {
                    Meta::Path(path) => {
                        if path.is_ident("default") {
                            Some(DefaultToSet::AsDefault)
                        } else {
                            None
                        }
                    }
                    Meta::List(_) => None,
                    Meta::NameValue(nv) => {
                        if nv.path.is_ident("default") {
                            Some(DefaultToSet::AsValue(nv.value.to_token_stream()))
                        } else {
                            None
                        }
                    }
                }),
                Err(_) => None,
            }
        })
}
