//! Variant parsing.

use syn::{Fields, Ident, Result};

use crate::{Field, VariantAttr};

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
}
