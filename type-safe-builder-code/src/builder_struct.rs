use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Generics;

pub(super) fn create(builder_factory_ident: &Ident, generics: &Generics) -> TokenStream {
    let generic_params = generics.params.iter().clone();

    let all_phantom_fields = generics.params.iter().enumerate().map(|(i, gen)| {
        let gen_ident = format_ident!("phantom_field_{}", i);
        quote! {
            #gen_ident: core::marker::PhantomData<#gen>
        }
    });

    quote! {
        struct #builder_factory_ident <#(#generic_params,)*> {
            #(#all_phantom_fields,)*
        }
    }
}
