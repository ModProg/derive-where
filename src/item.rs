//! Intermediate representation of item data.

use syn::Ident;

use crate::{attr::Incomparable, Data, Trait};

/// Fields or variants of an item.
#[cfg_attr(test, derive(Debug))]
#[allow(clippy::large_enum_variant)]
pub enum Item<'a> {
	/// Enum.
	Enum {
		/// [`struct@Ident`] of this enum.
		ident: &'a Ident,
		/// [`Incomparable`] attribute of this enum.
		incomparable: Incomparable,
		/// Variants of this enum.
		variants: Vec<Data<'a>>,
	},
	/// Struct, tuple struct or union.
	Item(Data<'a>),
}

impl Item<'_> {
	/// Returns [`struct@Ident`] of this [`Item`].
	pub fn ident(&self) -> &Ident {
		match self {
			Item::Item(data) => data.ident,
			Item::Enum { ident, .. } => ident,
		}
	}

	/// Returns `true` if this [`Item`] if an enum.
	pub fn is_enum(&self) -> bool {
		match self {
			Item::Enum { .. } => true,
			Item::Item(_) => false,
		}
	}

	/// Returns `true` if any field is skipped with that [`Trait`].
	pub fn any_skip_trait(&self, trait_: Trait) -> bool {
		match self {
			Item::Item(data) => data.any_skip_trait(trait_),
			Item::Enum { variants, .. } => variants.iter().any(|data| data.any_skip_trait(trait_)),
		}
	}

	/// Returns `true` if any field uses `Zeroize(fqs)`.
	#[cfg(feature = "zeroize")]
	pub fn any_fqs(&self) -> bool {
		use crate::Either;

		match self {
			Item::Item(data) => match data.fields() {
				Either::Left(fields) => fields.fields.iter().any(|field| field.attr.zeroize_fqs.0),
				Either::Right(_) => false,
			},
			Item::Enum { variants, .. } => variants.iter().any(|data| match data.fields() {
				Either::Left(fields) => fields.fields.iter().any(|field| field.attr.zeroize_fqs.0),
				Either::Right(_) => false,
			}),
		}
	}

	/// Returns `true` if all [`Fields`](crate::data::Fields) are empty for this
	/// [`Trait`].
	pub fn is_empty(&self, trait_: Trait) -> bool {
		match self {
			Item::Enum { variants, .. } => variants.iter().all(|data| data.is_empty(trait_)),
			Item::Item(data) => data.is_empty(trait_),
		}
	}

	/// Returns `true` if the item is incomparable or all (≥1) variants are
	/// incomparable.
	pub fn is_incomparable(&self) -> bool {
		match self {
			Item::Enum {
				variants,
				incomparable,
				..
			} => {
				incomparable.0.is_some()
					|| !variants.is_empty() && variants.iter().all(Data::is_incomparable)
			}
			Item::Item(data) => data.is_incomparable(),
		}
	}
}
