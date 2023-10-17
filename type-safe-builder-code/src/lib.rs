use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse2, DataStruct, DeriveInput, Field, FieldsNamed};

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

    let builder_struct = builder_struct(&builder_factory_ident);
    let builder_factory_impl =
        builder_factory_impl(fields, &builder_factory_ident, &builder_state_ident);
    let builder_state_struct = builder_state_struct(fields, &builder_state_ident);
    let all_field_setter_impl = all_field_setters_impl(fields, &builder_state_ident);
    let build_impl = build_impl(fields, &builder_state_ident, &name);

    quote! {
        #builder_struct
        #builder_factory_impl
        #builder_state_struct

        #(#all_field_setter_impl )*

        #build_impl
    }
}

fn builder_struct(builder_factory_ident: &Ident) -> TokenStream {
    quote! {
        struct #builder_factory_ident {}
    }
}

fn builder_factory_impl(
    fields: &Punctuated<Field, Comma>,
    builder_factory_ident: &Ident,
    builder_state_ident: &Ident,
) -> TokenStream {
    let all_unset_fields = fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();
        let field_ident = format_ident!("{}", field_name);
        let phantom_field_ident = format_ident!("phantom_{}", field_name);

        quote! {
            #field_ident: None,
            #phantom_field_ident: Default::default()
        }
    });

    let all_unset = fields.iter().map(|_| {
        quote! {
            type_safe_builder_code::Unset
        }
    });

    quote! {
        impl #builder_factory_ident {
            pub fn builder() -> #builder_state_ident<#(#all_unset,)*> {
                #builder_state_ident {
                    #(#all_unset_fields,)*
                }
            }
        }
    }
}

fn builder_state_struct(
    fields: &Punctuated<Field, Comma>,
    builder_state_ident: &Ident,
) -> TokenStream {
    let state_fields_declarations = fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();
        let field_type = &field.ty;
        let field_ident = format_ident!("{}", field_name);
        let phantom_field_ident = format_ident!("phantom_{}", field_name);
        let phantom_field_type_ident = format_ident!("Phantom{}Type", field_name);
        quote! {
            #field_ident: Option<#field_type>,
            #phantom_field_ident: core::marker::PhantomData<#phantom_field_type_ident>
        }
    });

    let all_phantom_fields_types = fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();
        let phantom_field_type_ident = phantom_field_type_ident(field_name);
        quote! {
            #phantom_field_type_ident
        }
    });

    quote! {
        struct #builder_state_ident<#(#all_phantom_fields_types,)*> {
            #(#state_fields_declarations,)*
        }
    }
}

fn all_field_setters_impl(
    fields: &Punctuated<Field, Comma>,
    builder_state_ident: &Ident,
) -> Vec<TokenStream> {
    fields.iter().map(|field| {
        let field_name = &field.ident.clone().unwrap();
        let field_type = &field.ty;
        let field_ident = format_ident!("{}", field_name);

        let other_phantom_field_type_ident = fields.iter().filter_map(|field| {
            if field.ident.clone().unwrap().to_string() == field_name.to_string() {
                None
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().unwrap());
                Some(quote!{#phantom_field_type_ident})
            }
        });

        let input_phantom_field_type_ident = fields.iter().map(|field| {
            if field.ident.clone().unwrap().to_string() == field_name.to_string() {
                quote!{type_safe_builder_code::Unset}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().unwrap());
                quote!{#phantom_field_type_ident}
            }
        });

        let output_phantom_field_type_ident = fields.iter().map(|field| {
            if field.ident.clone().unwrap().to_string() == field_name.to_string() {
                quote!{#field_type}
            } else {
                let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().unwrap());
                quote!{#phantom_field_type_ident}
            }
        });

        let copy_other_fields = fields.iter().filter_map(|field| {
            if field.ident.clone().unwrap().to_string() == field_name.to_string() {
                None
            } else {
                let field_name = &field.ident.clone().unwrap();
                Some(quote!{#field_name: self.#field_name})
            }
        });

        let all_phantom_to_default = fields.iter().map(|field| {
            let field_name = &field.ident.clone().unwrap();
            let phantom_field_ident = format_ident!("phantom_{}", field_name);

            quote! {
            #phantom_field_ident: Default::default()
        }
        });

        quote! {
            impl<#(#other_phantom_field_type_ident,)*> #builder_state_ident<#(#input_phantom_field_type_ident,)*> {
            fn #field_ident(self, value: #field_type) -> #builder_state_ident<#(#output_phantom_field_type_ident,)*> {
                #builder_state_ident {
                    #field_ident: Some(value),
                    #(#copy_other_fields,)*
                    #(#all_phantom_to_default,)*
                    }
                }
            }
        }
    }).collect()
}

fn build_impl(
    fields: &Punctuated<Field, Comma>,
    builder_state_ident: &Ident,
    name: &Ident,
) -> TokenStream {
    let all_not_default_set = fields.iter().map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().unwrap());
        let field_type = &field.ty;
        if is_with_default(field) {
            quote! {
                #phantom_field_type_ident
            }
        } else {
            quote! {
                #field_type
            }
        }
    });

    let all_default_phantom_fields_types = fields.iter().filter_map(|field| {
        let phantom_field_type_ident = phantom_field_type_ident(&field.ident.clone().unwrap());
        if is_with_default(field) {
            Some(quote! {#phantom_field_type_ident})
        } else {
            None
        }
    });

    let copy_all_fields = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        if is_with_default(field) {
            default_to_set(field)
                .map(|t| {
                    quote! {
                        #field_name: #t
                    }
                })
                .unwrap_or(quote! {
                    #field_name: self.#field_name.unwrap_or_default()
                })
        } else {
            quote! {
                #field_name: self.#field_name.unwrap()
            }
        }
    });

    quote! {
        impl <#(#all_default_phantom_fields_types,)*> #builder_state_ident<#(#all_not_default_set,)*> {
            fn build(self) -> #name {
                #name {
                    #(#copy_all_fields,)*
                }
            }
        }
    }
}

fn is_with_default(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("build_default"))
}

fn default_to_set(field: &Field) -> Option<TokenStream> {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("build_default"))
        .map(|attr| attr.parse_args::<TokenStream>().ok())
        .flatten()
}

fn phantom_field_type_ident(field_name: &Ident) -> Ident {
    format_ident!("Phantom{}Type", field_name)
}

#[test]
fn test() {
    let input = quote! {
        struct Struct1 {
            #[build_default]
            field1: i64,
            field2: String,
        }
    };

    let actual = builder_for(input);
    println!("{}", actual.to_string());
    assert!(actual.to_string().contains("struct Struct1Builder"));
}
