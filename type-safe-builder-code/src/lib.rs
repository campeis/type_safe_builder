use proc_macro2::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests;

mod all_field_setters_impl;
mod build_impl;
mod builder_factory_impl;
mod builder_state_struct;
mod builder_struct;
mod parse;

pub fn builder_for(item: TokenStream) -> TokenStream {
    let from_struct = parse::parse(item);
    let builder_struct = builder_struct::create(&from_struct);
    let builder_factory_impl = builder_factory_impl::create(&from_struct);
    let builder_state_struct = builder_state_struct::create(&from_struct);
    let all_field_setter_impl = all_field_setters_impl::create(&from_struct);
    let build_impl = build_impl::create(&from_struct);

    quote! {
        #builder_struct
        #builder_factory_impl
        #builder_state_struct

        #(#all_field_setter_impl )*

        #build_impl
    }
}
