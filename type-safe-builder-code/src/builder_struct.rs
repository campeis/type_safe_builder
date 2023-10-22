use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{GenericParam, Generics};

pub(super) fn create(builder_factory_ident: &Ident, generics: &Generics) -> TokenStream {
    let generic_params = generics.params.iter().clone();
    let where_clause = generics.where_clause.clone().map(|clause| {
        quote! {
            #clause
        }
    });

    let all_phantom_fields = generics.params.iter().enumerate().map(|(i, gen)| {
        let gen_ident = format_ident!("phantom_field_{}", i);

        let gen_types = match gen {
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
        };

        quote! {
            #gen_ident: core::marker::PhantomData<#gen_types>
        }
    });

    quote! {
        struct #builder_factory_ident <#(#generic_params,)*> #where_clause{
            #(#all_phantom_fields,)*
        }
    }
}
