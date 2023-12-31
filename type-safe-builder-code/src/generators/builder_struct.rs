use crate::parse::FromStruct;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn create(from_struct: &FromStruct) -> TokenStream {
    let builder_factory_ident = from_struct.builder_ident();
    quote! {
        struct #builder_factory_ident {
        }
    }
}
