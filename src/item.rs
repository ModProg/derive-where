//! Intermediate representation of item data.

use syn::Ident;

use crate::{Data, DeriveWhere, Trait};

/// Fields or variants of an item.
#[allow(clippy::large_enum_variant)]
pub enum Item<'a> {
	/// Enum.
	Enum {
		/// [`struct@Ident`] of this enum.
		ident: &'a Ident,
		/// Variants of this enum.
		variants: Vec<Data<'a>>,
	},
	/// Struct, tuple struct or union.
	Item(Data<'a>),
}

impl Item<'_> {
	/// Return [`struct@Ident`] of this [`Item`].
	pub fn ident(&self) -> &Ident {
		match self {
			Item::Item(data) => data.ident,
			Item::Enum { ident, .. } => ident,
		}
	}

	/// Returns `true` if any field is skipped with that [`Trait`].
	pub fn skip(&self, trait_: &Trait) -> bool {
		match self {
			Item::Item(data) => data.skip(trait_),
			Item::Enum { variants, .. } => variants.iter().any(|data| data.skip(trait_)),
		}
	}

	/// Returns `true` if any field is skipped.
	pub fn any_skip(&self) -> bool {
		match self {
			Item::Item(data) => data.any_skip(),
			Item::Enum { variants, .. } => variants.iter().any(|data| data.any_skip()),
		}
	}

	/// Returns `true` if any field uses `default`.
	// MSRV: `matches!` was added in 1.42.0.
	#[allow(clippy::match_like_matches_macro)]
	pub fn any_default(&self, derive_wheres: &[DeriveWhere]) -> bool {
		match self {
			Item::Enum { .. } => derive_wheres
				.iter()
				.any(|derive_where| derive_where.trait_(Trait::Default).is_some()),
			_ => false,
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
}
