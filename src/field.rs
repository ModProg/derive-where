//! Field parsing.

use proc_macro2::Span;
use syn::{Attribute, FieldsNamed, FieldsUnnamed, Ident, Index, Result};

use crate::FieldAttr;

/// Struct and struct variant field.
pub struct Field<'a> {
    /// Attributes.
    attr: FieldAttr,
    /// [`Ident`](syn::Ident) or [`Index`](syn::Index) for this field.
    member: Member<'a>,
}

/// Borrowed version of [`syn::Member`] to avoid unnecessary allocations.
enum Member<'a> {
    /// Named field.
    Named(&'a Ident),
    /// Unnamed field.
    Unnamed(Index),
}

impl<'a> Field<'a> {
    /// Create [`Field`]s from [`syn::FieldsNamed`].
    pub fn from_named(fields: &'a FieldsNamed) -> Result<Vec<Self>> {
        let mut output = Vec::with_capacity(fields.named.len());

        for field in &fields.named {
            output.push(Self::from_field(
                &field.attrs,
                Member::Named(field.ident.as_ref().expect("unexpected unnamed field")),
            )?);
        }

        Ok(output)
    }

    /// Create [`Field`]s from [`syn::FieldsUnnamed`].
    pub fn from_unnamed(fields: &'a FieldsUnnamed) -> Result<Vec<Self>> {
        let mut output = Vec::with_capacity(fields.unnamed.len());

        for (index, field) in (0_u32..).zip(&fields.unnamed) {
            output.push(Self::from_field(
                &field.attrs,
                Member::Unnamed(Index {
                    index,
                    span: Span::call_site(),
                }),
            )?);
        }

        Ok(output)
    }

    /// Create [`Field`] from [`syn::Field`].
    fn from_field(attrs: &[Attribute], member: Member<'a>) -> Result<Self> {
        let attr = FieldAttr::from_attrs(attrs)?;

        Ok(Self { attr, member })
    }
}
