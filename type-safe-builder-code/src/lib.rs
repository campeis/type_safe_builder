use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse2, DataStruct, DeriveInput, FieldsNamed};

#[cfg(test)]
mod tests;

mod all_field_setters_impl;
mod build_impl;
mod builder_factory_impl;
mod builder_state_struct;
mod builder_struct;

pub struct Unset {}

pub fn builder_for(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(item).unwrap();
    let name = ast.ident;
    let builder_factory_ident = format_ident!("{}Builder", name);
    let builder_state_ident = format_ident!("{}BuilderState", name);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only implemented for structs"),
    };

    let builder_struct = builder_struct::create(&builder_factory_ident);
    let builder_factory_impl =
        builder_factory_impl::create(fields, &builder_factory_ident, &builder_state_ident);
    let builder_state_struct = builder_state_struct::create(fields, &builder_state_ident);
    let all_field_setter_impl = all_field_setters_impl::create(fields, &builder_state_ident);
    let build_impl = build_impl::create(fields, &builder_state_ident, &name);

    quote! {
        #builder_struct
        #builder_factory_impl
        #builder_state_struct

        #(#all_field_setter_impl )*

        #build_impl
    }
}
