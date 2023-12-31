use proc_macro2::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests;

mod generators;
mod parse;

pub fn builder_for(item: TokenStream) -> TokenStream {
    let from_struct = parse::parse(item);
    let builder_struct = generators::builder_struct::create(&from_struct);
    let builder_factory_impl = generators::factory::create(&from_struct);
    let builder_state_struct = generators::state_struct::create(&from_struct);
    let all_field_setter_impl = generators::all_field_setters::create(&from_struct);
    let build_impl = generators::build::create(&from_struct);

    quote! {
        #builder_struct
        #builder_factory_impl
        #builder_state_struct

        #(#all_field_setter_impl )*

        #build_impl
    }
}
