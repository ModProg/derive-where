//! Parses [`DeriveInput`] into something more useful.

use proc_macro2::Span;
use syn::{DeriveInput, Generics, Result};

use crate::{Data, Default, DeriveWhere, Either, Error, Item, ItemAttr, Trait, VariantAttr};

/// Parsed input.
pub struct Input<'a> {
	/// `derive_where` attributes on the item.
	pub derive_wheres: Vec<DeriveWhere>,
	/// Generics necessary to define for an `impl`.
	pub generics: &'a Generics,
	/// Fields or variants of this item.
	pub item: Item<'a>,
}

impl<'a> Input<'a> {
	/// Create [`Input`] from `proc_macro_derive` parameter.
	pub fn parse(
		span: Span,
		DeriveInput {
			attrs,
			ident,
			generics,
			data,
			..
		}: &'a DeriveInput,
	) -> Result<Self> {
		// Parse `Attribute`s on item.
		let ItemAttr {
			skip_inner,
			derive_wheres,
		} = ItemAttr::from_attrs(span, data, attrs)?;

		// Extract fields and variants of this item.
		let item = match &data {
			syn::Data::Struct(data) => {
				Data::from_struct(span, skip_inner, ident, &data.fields).map(Item::Item)?
			}
			syn::Data::Enum(data) => {
				let mut accumulated_defaults = Default::default();

				let variants = data
					.variants
					.iter()
					.map(|variant| {
						// Parse `Attribute`s on variant.
						let VariantAttr {
							default,
							skip_inner,
						} = VariantAttr::from_attrs(
							&variant.attrs,
							&derive_wheres,
							&mut accumulated_defaults,
							variant,
						)?;

						Data::from_variant(
							ident,
							skip_inner,
							default,
							&variant.ident,
							&variant.fields,
						)
					})
					.collect::<Result<Vec<Data>>>()?;

				// Empty enums aren't allowed.
				if variants.iter().all(|variant| match variant.fields() {
					Either::Left(fields) => fields.fields.is_empty(),
					Either::Right(_) => true,
				}) {
					return Err(Error::item_empty(span));
				}

				// Make sure a variant has the `option` attribute if `Default` is being
				// implemented.
				if !accumulated_defaults.0
					&& derive_wheres.iter().any(|derive_where| {
						derive_where
							.traits
							.iter()
							.any(|trait_| **trait_ == Trait::Default)
					}) {
					return Err(Error::default_missing(span));
				}

				Item::Enum { ident, variants }
			}
			syn::Data::Union(data) => {
				Data::from_union(span, skip_inner, ident, &data.fields).map(Item::Item)?
			}
		};

		#[cfg(feature = "zeroize")]
		{
			if !(
				// Any generics used.
				!generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
                // `Zeroize(fqs)` is used on any field.
                || item.any_fqs()
			) {
				return Err(Error::item(span));
			}
		}

		#[cfg(not(feature = "zeroize"))]
		{
			if !(
				// Any generics used.
				!generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
			) {
				return Err(Error::item(span));
			}
		}

		Ok(Self {
			derive_wheres,
			generics,
			item,
		})
	}
}
