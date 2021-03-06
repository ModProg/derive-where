//! Parses [`DeriveInput`] into something more useful.

use proc_macro2::Span;
use syn::{DeriveInput, GenericParam, Generics, Result};

#[cfg(feature = "zeroize")]
use crate::DeriveTrait;
use crate::{Data, DeriveWhere, Either, Error, Item, ItemAttr, Trait};

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
	pub fn from_input(
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
				Data::from_struct(span, &derive_wheres, skip_inner, ident, &data.fields)
					.map(Item::Item)?
			}
			syn::Data::Enum(data) => {
				let variants = data
					.variants
					.iter()
					.map(|variant| Data::from_variant(ident, &derive_wheres, variant))
					.collect::<Result<Vec<Data>>>()?;

				// Find if a default option is specified on a variant.
				let mut found_default = false;

				// While searching for a default option, check for duplicates.
				for variant in &variants {
					if let Some(span) = variant.default_span() {
						if found_default {
							return Err(Error::default_duplicate(span));
						} else {
							found_default = true;
						}
					}
				}

				// Make sure a variant has the `option` attribute if `Default` is being
				// implemented.
				if !found_default
					&& derive_wheres
						.iter()
						.any(|derive_where| derive_where.contains(Trait::Default))
				{
					return Err(Error::default_missing(span));
				}

				// Empty enums aren't allowed unless they implement `Default`.
				if !found_default
					&& variants.iter().all(|variant| match variant.fields() {
						Either::Left(fields) => fields.fields.is_empty(),
						Either::Right(_) => true,
					}) {
					return Err(Error::item_empty(span));
				}

				Item::Enum { ident, variants }
			}
			syn::Data::Union(data) => {
				Data::from_union(span, &derive_wheres, skip_inner, ident, &data.fields)
					.map(Item::Item)?
			}
		};

		// Don't allow generic constraints be the same as generics on item unless there
		// is a use-case for it.
		// Count number of generic type parameters.
		let generics_len = generics
			.params
			.iter()
			.filter(|generic_param| match generic_param {
				GenericParam::Type(_) => true,
				GenericParam::Lifetime(_) | GenericParam::Const(_) => false,
			})
			.count();

		'outer: for derive_where in &derive_wheres {
			// No point in starting to compare both if not even the length is the same.
			// This can be easily circumvented by doing the following:
			// `#[derive_where(..; T: Clone)]`, or `#[derive_where(..; T, T)]`, which
			// apparently is valid Rust syntax: `where T: Clone, T: Clone`, we are only here
			// to help though.
			if derive_where.generics.len() != generics_len {
				continue;
			}

			// No point in starting to check if there is no use-case if a custom bound was
			// used, which is a use-case.
			if derive_where.any_custom_bound() {
				continue;
			}

			// Check if every generic type parameter present on the item is defined in this
			// `DeriveWhere`.
			for generic_param in &generics.params {
				// Only check generic type parameters.
				if let GenericParam::Type(type_param) = generic_param {
					if !derive_where.has_type_param(&type_param.ident) {
						continue 'outer;
					}
				}
			}

			// The `for` loop should short-circuit to the `'outer` loop if not all generic
			// type parameters were found.

			// Don't allow no use-case compared to std `derive`.
			for (span, trait_) in derive_where.spans.iter().zip(&derive_where.traits) {
				// `Default` is used on an enum.
				if trait_ == Trait::Default && item.is_enum() {
					continue;
				}

				// Any field is skipped with a corresponding `Trait`.
				if item.any_skip_trait(**trait_) {
					continue;
				}

				#[cfg(feature = "zeroize")]
				{
					// `Zeroize(crate = "..")` or `ZeroizeOnDrop(crate = "..")` is used.
					if let DeriveTrait::Zeroize { crate_: Some(_) }
					| DeriveTrait::ZeroizeOnDrop { crate_: Some(_) } = *trait_
					{
						continue;
					}

					// `Zeroize(fqs)` is used on any field.
					if trait_ == Trait::Zeroize && item.any_fqs() {
						continue;
					}
				}

				return Err(Error::use_case(*span));
			}
		}

		Ok(Self {
			derive_wheres,
			generics,
			item,
		})
	}
}
