use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::Data::Struct;
use syn::Fields::Named;
use syn::{
    parse2, Attribute, DataStruct, DeriveInput, FieldsNamed, GenericParam, Meta, Token, Type,
    WhereClause,
};

pub(crate) struct FromStruct {
    pub(crate) ident: Ident,
    pub(crate) generics: StructGenerics,
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
    is_default_as_standard: bool,
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

    pub(crate) fn field_placeholder(&self) -> TokenStream {
        format_ident!("PLACEHOLDER{}TYPE", self.ident.to_string().to_uppercase()).to_token_stream()
    }

    pub(crate) fn const_field_placeholder(&self) -> TokenStream {
        let field_placeholder = self.field_placeholder();
        quote! {const #field_placeholder: bool}
    }

    pub(crate) fn has_default(&self) -> bool {
        self.default_to_set().is_some()
    }
    pub(crate) fn default_to_set(&self) -> Option<DefaultToSet> {
        if self.has_mandatory() {
            return None;
        }
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
            .or_else(|| {
                if self.is_default_as_standard {
                    Some(DefaultToSet::AsDefault)
                } else {
                    None
                }
            })
    }

    pub(crate) fn has_mandatory(&self) -> bool {
        self.attrs
            .iter()
            .find(|attr| attr.path().is_ident("builder"))
            .and_then(|attr| {
                let values: Result<Punctuated<Meta, Token![,]>, _> =
                    attr.parse_args_with(Punctuated::parse_terminated);

                match values {
                    Ok(values) => values.iter().find_map(|m| match m {
                        Meta::Path(path) => {
                            if path.is_ident("mandatory") {
                                Some(true)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }),
                    Err(_) => None,
                }
            })
            .unwrap_or_default()
    }
}

pub(crate) struct StructGenerics {
    where_clause: Option<WhereClause>,
    generics: Vec<GenericParam>,
}

impl StructGenerics {
    pub(crate) fn where_clause(&self) -> Option<TokenStream> {
        self.where_clause.clone().map(|clause| {
            quote! {
                #clause
            }
        })
    }

    pub(crate) fn all(&self) -> Vec<TokenStream> {
        self.generics
            .iter()
            .map(|param| {
                quote! {
                    #param
                }
            })
            .collect()
    }

    pub(crate) fn all_names(&self) -> Vec<TokenStream> {
        self.generics
            .iter()
            .map(|gen| match gen {
                GenericParam::Lifetime(l) => {
                    let l = &l.lifetime;
                    quote! {#l}
                }
                GenericParam::Type(t) => {
                    let i = &t.ident;
                    quote! {#i}
                }
                GenericParam::Const(c) => {
                    let i = &c.ident;
                    quote! {#i}
                }
            })
            .collect()
    }
}

pub(super) fn parse(item: TokenStream) -> FromStruct {
    let ast: DeriveInput = parse2(item).unwrap();

    let is_default_as_standard = has_attr_path(&ast, "default");

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named.iter().map(|field| Field {
            ident: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
            attrs: field.attrs.clone(),
            is_default_as_standard,
        }),
        _ => unimplemented!("Only implemented for structs"),
    }
    .collect::<Vec<_>>();

    FromStruct {
        ident: ast.ident,
        generics: StructGenerics {
            generics: ast.generics.params.into_iter().collect(),
            where_clause: ast.generics.where_clause,
        },
        fields,
    }
}

fn has_attr_path(ast: &DeriveInput, attr_path: &str) -> bool {
    ast.attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
        .map(|attr| {
            let values: Result<Punctuated<Meta, Token![,]>, _> =
                attr.parse_args_with(Punctuated::parse_terminated);

            match values {
                Ok(values) => values.iter().any(|m| match m {
                    Meta::Path(path) => path.is_ident(&format_ident!("{}", attr_path)),
                    Meta::List(_) => false,
                    Meta::NameValue(_) => false,
                }),
                Err(_) => false,
            }
        })
        .unwrap_or_default()
}
