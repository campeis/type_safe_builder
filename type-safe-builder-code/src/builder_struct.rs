use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub(super) fn create(builder_factory_ident: &Ident) -> TokenStream {
    quote! {
        struct #builder_factory_ident {}
    }
}
