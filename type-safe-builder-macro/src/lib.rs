extern crate proc_macro;

use proc_macro::TokenStream;
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(item: TokenStream) -> TokenStream {
    type_safe_builder_code::builder_for(item.into()).into()
}
