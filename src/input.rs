//! Saves an intermediate representation of the input.

use proc_macro2::Span;
use syn::{DeriveInput, Fields, Generics, Ident, Result};

use crate::{Error, Field, ItemAttr, Trait, Variant};

/// Parsed input.
pub struct Input<'a> {
    /// `derive_where` attributes on the item.
    attrs: ItemAttr,
    /// Item [`Ident`].
    pub ident: &'a Ident,
    /// Item [`Span`].
    pub span: Span,
    /// Generics necessary to define for an `impl`.
    pub generics: &'a Generics,
    /// Fields or variants of this item.
    pub data: Data<'a>,
}

/// Fields or variants of an item.
pub enum Data<'a> {
    /// Struct.
    Struct(Vec<Field<'a>>),
    /// Tuple struct.
    Tuple(Vec<Field<'a>>),
    /// Enum.
    Enum(Vec<Variant<'a>>),
    /// Union.
    Union(Vec<Field<'a>>),
}

impl<'a> Input<'a> {
    /// Create [`Input`] from `proc_macro_derive` parameter.
    pub fn parse(span: Span, item: &'a DeriveInput) -> Result<Self> {
        /*// Save `Span` before we consume `input` when parsing it.
        let span = input.span();
        let item = syn::parse2::<DeriveInput>(input).expect("derive on unexpected item");*/

        // Parse `Attribute`s on item.
        let attrs = ItemAttr::from_attrs(&item.attrs)?;

        // Extract fields and variants of this item.
        let data = match &item.data {
            syn::Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => Field::from_named(fields).map(Data::Struct),
                Fields::Unnamed(fields) => Field::from_unnamed(fields).map(Data::Tuple),
                Fields::Unit => Err(Error::unit_struct(span)),
            }?,
            syn::Data::Enum(data) => {
                let variants = data
                    .variants
                    .iter()
                    .map(Variant::from_variant)
                    .collect::<Result<_>>()?;

                Data::Enum(variants)
            }
            syn::Data::Union(data) => Data::Union(Field::from_named(&data.fields)?),
        };

        Ok(Self {
            attrs,
            ident: &item.ident,
            span,
            generics: &item.generics,
            data,
        })
    }
}

impl<'a> Data<'a> {
    /// Returns `true` if all variants or fields are skipped with the given [`Trait`].
    pub fn skip(&self, trait_: &Trait) -> bool {
        match self {
            Data::Struct(fields) | Data::Tuple(fields) => {
                fields.iter().all(|field| field.skip(trait_))
            }
            Data::Enum(variants) => variants.iter().all(|variant| variant.skip(trait_)),
            // `union`s don't support skip.
            Data::Union(_) => false,
        }
    }
}
