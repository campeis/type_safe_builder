use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, ToTokens};
use syn::punctuated::Punctuated;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{parse2, Attribute, DataStruct, DeriveInput, FieldsNamed, Generics, Meta, Token, Type};

pub(crate) struct FromStruct {
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) fields: Vec<Field>,
}

impl FromStruct {
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }
    pub(crate) fn builder_ident(&self) -> Ident {
        format_ident!("{}Builder", self.ident)
    }

    pub(crate) fn builder_state_ident(&self) -> Ident {
        format_ident!("{}BuilderState", self.ident)
    }
}

pub(crate) struct Field {
    ident: Ident,
    ty: Type,
    attrs: Vec<Attribute>,
}

pub(crate) enum DefaultToSet {
    AsDefault,
    AsValue(TokenStream),
}
impl Field {
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }

    pub(crate) fn ty(&self) -> &Type {
        &self.ty
    }

    pub(crate) fn field_placeholder(&self) -> Ident {
        format_ident!("PLACEHOLDER{}TYPE", self.ident.to_string().to_uppercase())
    }

    pub(crate) fn has_default(&self) -> bool {
        self.default_to_set().is_some()
    }
    pub(crate) fn default_to_set(&self) -> Option<DefaultToSet> {
        self.attrs
            .iter()
            .find(|attr| attr.path().is_ident("builder"))
            .and_then(|attr| {
                let values: Result<Punctuated<Meta, Token![,]>, _> =
                    attr.parse_args_with(Punctuated::parse_terminated);

                match values {
                    Ok(values) => values.iter().find_map(|m| match m {
                        Meta::Path(path) => {
                            if path.is_ident("default") {
                                Some(DefaultToSet::AsDefault)
                            } else {
                                None
                            }
                        }
                        Meta::List(_) => None,
                        Meta::NameValue(nv) => {
                            if nv.path.is_ident("default") {
                                Some(DefaultToSet::AsValue(nv.value.to_token_stream()))
                            } else {
                                None
                            }
                        }
                    }),
                    Err(_) => None,
                }
            })
    }
}

pub(super) fn parse(item: TokenStream) -> FromStruct {
    let ast: DeriveInput = parse2(item).unwrap();

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named.iter().map(|field| Field {
            ident: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
            attrs: field.attrs.clone(),
        }),
        _ => unimplemented!("Only implemented for structs"),
    }
    .collect::<Vec<_>>();

    FromStruct {
        ident: ast.ident,
        generics: ast.generics,
        fields,
    }
}
