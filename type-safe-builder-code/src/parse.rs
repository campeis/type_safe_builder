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
    builder_ident_name: Option<Ident>,
}

impl FromStruct {
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }
    pub(crate) fn builder_ident(&self) -> Ident {
        self.builder_ident_name
            .clone()
            .unwrap_or_else(|| format_ident!("{}Builder", self.ident))
    }

    pub(crate) fn builder_state_ident(&self) -> Ident {
        self.builder_ident_name
            .clone()
            .map(|builder_ident| format_ident!("{}State", builder_ident))
            .unwrap_or_else(|| format_ident!("{}BuilderState", self.ident))
    }
}

pub(crate) struct Field {
    ident: Ident,
    ty: Type,
    attrs: Vec<Attribute>,
    is_default_as_standard: bool,
    is_default_as_multi: bool,
}

pub(crate) enum DefaultToSet {
    AsDefault,
    AsValue(TokenStream),
}
impl Field {
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }

    pub(crate) fn setter_ident(&self) -> Option<TokenStream> {
        self.get_attr_value("setter_name")
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
        self.get_attr_value("default")
            .map(DefaultToSet::AsValue)
            .or_else(|| {
                if self.has_attr_path("default") || self.is_default_as_standard {
                    Some(DefaultToSet::AsDefault)
                } else {
                    None
                }
            })
    }

    fn has_mandatory(&self) -> bool {
        self.has_attr_path("mandatory")
    }

    pub(crate) fn has_multi(&self) -> bool {
        (self.is_default_as_multi && !self.has_single()) || self.has_attr_path("multi")
    }

    fn has_single(&self) -> bool {
        self.has_attr_path("single")
    }

    fn has_attr_path(&self, path_to_find: &'static str) -> bool {
        has_attr_path(&self.attrs, path_to_find)
    }

    fn get_attr_value(&self, key: &'static str) -> Option<TokenStream> {
        get_attr_value(&self.attrs, key)
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

    let is_default_as_standard = has_attr_path(&ast.attrs, "default");
    let is_default_as_multi = has_attr_path(&ast.attrs, "multi");

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named.iter().map(|field| Field {
            ident: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
            attrs: field.attrs.clone(),
            is_default_as_standard,
            is_default_as_multi,
        }),
        _ => unimplemented!("Only implemented for structs"),
    }
    .collect::<Vec<_>>();

    let builder_ident_name =
        get_attr_value(&ast.attrs, "name").map(|ts| format_ident!("{}", ts.to_string()));

    FromStruct {
        ident: ast.ident,
        generics: StructGenerics {
            generics: ast.generics.params.into_iter().collect(),
            where_clause: ast.generics.where_clause,
        },
        fields,
        builder_ident_name,
    }
}

fn has_attr_path(attrs: &[Attribute], attr_path: &str) -> bool {
    attrs
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

fn get_attr_value(attrs: &[Attribute], key: &str) -> Option<TokenStream> {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("builder"))
        .iter()
        .find_map(|attr| {
            let values: Result<Punctuated<Meta, Token![,]>, _> =
                attr.parse_args_with(Punctuated::parse_terminated);

            match values {
                Ok(values) => values.iter().find_map(|m| match m {
                    Meta::Path(_) => None,
                    Meta::List(_) => None,
                    Meta::NameValue(nv) => {
                        if nv.path.is_ident(key) {
                            Some(nv.value.to_token_stream())
                        } else {
                            None
                        }
                    }
                }),
                Err(_) => None,
            }
        })
}
