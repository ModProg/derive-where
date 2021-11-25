//! Variant parsing.

use core::iter::FromIterator;

use syn::{
    punctuated::Punctuated,
    token::{Brace, Paren},
    Fields, Ident, Pat, PatPath, PatRest, PatStruct, PatTuple, PatTupleStruct, Path, PathArguments,
    PathSegment, Result, Token,
};

use crate::{Field, Trait, VariantAttr};

/// `enum` variant.
pub struct Variant<'a> {
    /// Attributes.
    pub attr: VariantAttr,
    /// [`Ident`] of this variant.
    pub ident: &'a Ident,
    /// [`Field`]s of this variant.
    pub data: VariantData<'a>,
}

/// Variant data of an enum.
pub enum VariantData<'a> {
    /// Struct variant.
    Struct(Vec<Field<'a>>),
    /// Tuple variant.
    Tuple(Vec<Field<'a>>),
    /// Unit variant.
    Unit,
}

impl<'a> Variant<'a> {
    /// Create [`Variant`] from [`syn::Variant`].
    pub fn from_variant(variant: &'a syn::Variant) -> Result<Self> {
        // Parse `Attribute`s on variant.
        let attr = VariantAttr::from_attrs(&variant.attrs)?;

        let data = match &variant.fields {
            Fields::Named(fields) => Field::from_named(fields).map(VariantData::Struct)?,
            Fields::Unnamed(fields) => Field::from_unnamed(fields).map(VariantData::Tuple)?,
            Fields::Unit => VariantData::Unit,
        };

        Ok(Variant {
            attr,
            ident: &variant.ident,
            data,
        })
    }

    /// Returns `true` if this variant is skipped with the given [`Trait`].
    pub fn skip(&self, trait_: &Trait) -> bool {
        self.attr.skip(trait_)
            || match &self.data {
                VariantData::Struct(fields) | VariantData::Tuple(fields) => {
                    fields.iter().all(|field| field.skip(trait_))
                }
                VariantData::Unit => false,
            }
    }

    /// Create `match` [pattern](Pat) to skip this variant.
    pub fn skip_pattern(&self, item: &Ident) -> Pat {
        let path = Path {
            leading_colon: None,
            segments: Punctuated::from_iter([item, self.ident].iter().map(|ident| PathSegment {
                ident: (*ident).clone(),
                arguments: PathArguments::None,
            })),
        };

        match self.data {
            VariantData::Struct(_) => Pat::Struct(PatStruct {
                attrs: Vec::new(),
                path,
                brace_token: Brace::default(),
                fields: Punctuated::default(),
                dot2_token: Some(<Token![..]>::default()),
            }),
            VariantData::Tuple(_) => Pat::TupleStruct(PatTupleStruct {
                attrs: Vec::new(),
                path,
                pat: PatTuple {
                    attrs: Vec::new(),
                    paren_token: Paren::default(),
                    elems: Punctuated::from_iter(Some(Pat::Rest(PatRest {
                        attrs: Vec::new(),
                        dot2_token: <Token![..]>::default(),
                    }))),
                },
            }),
            VariantData::Unit => Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path,
            }),
        }
    }
}
