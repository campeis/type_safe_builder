use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse2, Attribute, DataStruct, DeriveInput, FieldsNamed, Type};

#[cfg(test)]
mod tests;

mod all_field_setters_impl;
mod build_impl;
mod builder_factory_impl;
mod builder_state_struct;
mod builder_struct;

pub struct Set {}
pub struct Unset {}

struct NamedField {
    name: Ident,
    ty: Type,
    attrs: Vec<Attribute>,
}

pub fn builder_for(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse2(item).unwrap();
    let name = ast.ident;
    let builder_factory_ident = format_ident!("{}Builder", name);
    let builder_state_ident = format_ident!("{}BuilderState", name);
    let generics = ast.generics;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only implemented for structs"),
    };

    let fields = fields
        .iter()
        .map(|field| NamedField {
            name: field
                .ident
                .clone()
                .expect("Anonymous fields are not supported"),
            ty: field.ty.clone(),
            attrs: field.attrs.clone(),
        })
        .collect::<Vec<_>>();

    let builder_struct = builder_struct::create(&builder_factory_ident, &generics);
    let builder_factory_impl = builder_factory_impl::create(
        &fields,
        &builder_factory_ident,
        &builder_state_ident,
        &generics,
    );
    let builder_state_struct =
        builder_state_struct::create(&fields, &builder_state_ident, &generics);
    let all_field_setter_impl =
        all_field_setters_impl::create(&fields, &builder_state_ident, &generics);
    let build_impl = build_impl::create(&fields, &builder_state_ident, &generics, &name);

    quote! {
        #builder_struct
        #builder_factory_impl
        #builder_state_struct

        #(#all_field_setter_impl )*

        #build_impl
    }
}
