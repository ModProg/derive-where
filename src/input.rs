//! Parses [`DeriveInput`] into something more useful.

use proc_macro2::Span;
use syn::{DeriveInput, GenericParam, Generics, Result};

#[cfg(feature = "zeroize")]
use crate::DeriveTrait;
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
							&derive_wheres,
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
					&& derive_wheres
						.iter()
						.any(|derive_where| derive_where.trait_(&Trait::Default).is_some())
				{
					return Err(Error::default_missing(span));
				}

				Item::Enum { ident, variants }
			}
			syn::Data::Union(data) => {
				Data::from_union(span, &derive_wheres, skip_inner, ident, &data.fields)
					.map(Item::Item)?
			}
		};

		// Don't allow no use-case compared to std `derive`.
		let mut found_use_case = false;
		// Any generics used.
		found_use_case |= generics
			.params
			.iter()
			.any(|generic_param| match generic_param {
				GenericParam::Type(_) => true,
				GenericParam::Lifetime(_) | GenericParam::Const(_) => false,
			});
		// Any field is skipped.
		found_use_case |= item.any_skip();
		// `Default` is used on an enum.
		found_use_case |= item.any_default(&derive_wheres);
		#[cfg(feature = "zeroize")]
		{
			// `Zeroize(crate = "..")` is used.
			found_use_case |= derive_wheres.iter().any(|derive_where| {
				#[allow(clippy::match_like_matches_macro)]
				{
					if let Some(DeriveTrait::Zeroize {
						crate_: Some(_), ..
					}) = derive_where.trait_(&Trait::Zeroize)
					{
						true
					} else {
						false
					}
				}
			});
			// `Zeroize(fqs)` is used on any field.
			found_use_case |= item.any_fqs();
		}

		if !found_use_case {
			return Err(Error::item(span));
		}

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

			// Make sure we aren't using any other features.
			let mut found_use_case = false;
			// `Default` is used on an enum.
			found_use_case |= match item {
				Item::Enum { .. } => derive_where.trait_(&Trait::Default).is_some(),
				Item::Item(_) => false,
			};
			// Any field is skipped with a corresponding `Trait`.
			found_use_case |= derive_where
				.traits
				.iter()
				.any(|trait_| item.any_skip_trait(trait_));
			#[cfg(feature = "zeroize")]
			{
				// `Zeroize(crate = "..")` is used.
				found_use_case |= {
					#[allow(clippy::match_like_matches_macro)]
					{
						if let Some(DeriveTrait::Zeroize {
							crate_: Some(_), ..
						}) = derive_where.trait_(&Trait::Zeroize)
						{
							true
						} else {
							false
						}
					}
				};
				// `Zeroize(fqs)` is used on any field.
				found_use_case |= derive_where.trait_(&Trait::Zeroize).is_some() && item.any_fqs();
			}

			if !found_use_case {
				return Err(Error::generics(derive_where.span));
			}
		}

		Ok(Self {
			derive_wheres,
			generics,
			item,
		})
	}
}
