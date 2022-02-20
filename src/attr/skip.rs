//! Attribute parsing for the `skip` and `skip_inner` options.

use std::default::Default;

use syn::{spanned::Spanned, Meta, NestedMeta, Result};

use crate::{DeriveWhere, Error, Trait, TraitImpl};

/// Stores what [`Trait`]s to skip this field or variant for.
#[cfg_attr(test, derive(Debug))]
pub enum Skip {
	/// Field skipped for no [`Trait`].
	None,
	/// Field skipped for all [`Trait`]s that support it.
	All,
	/// Field skipped for the [`Trait`]s listed.
	Traits(Vec<Trait>),
}

impl Default for Skip {
	fn default() -> Self {
		Skip::None
	}
}

impl Skip {
	/// Token used for the `skip` option.
	pub const SKIP: &'static str = "skip";
	/// Token used for the `skip_inner` option.
	pub const SKIP_INNER: &'static str = "skip_inner";

	/// Returns `true` if variant is [`Skip::None`].
	pub fn is_none(&self) -> bool {
		matches!(self, Skip::None)
	}

	/// Adds a [`Meta`] to this [`Skip`].
	pub fn add_attribute(
		&mut self,
		derive_wheres: &[DeriveWhere],
		skip_inner: Option<&Skip>,
		meta: &Meta,
	) -> Result<()> {
		debug_assert!(meta.path().is_ident(Self::SKIP) || meta.path().is_ident(Self::SKIP_INNER));

		match meta {
			Meta::Path(path) => {
				// Check for duplicates.
				if self.is_none() {
					// Check against parent `skip_inner`.
					match skip_inner {
						// Allow `Skip::All` on field if parent has a tighter constraint.
						Some(Skip::None) | Some(Skip::Traits(..)) | None => {
							// Don't allow to skip all traits if no trait to be implemented supports
							// skipping.
							if derive_wheres
								.iter()
								.any(|derive_where| derive_where.any_skip())
							{
								*self = Skip::All;
								Ok(())
							} else {
								Err(Error::option_skip_no_trait(path.span()))
							}
						}
						// Don't allow `Skip::All` on field if parent already covers it.
						Some(Skip::All) => Err(Error::option_skip_inner(path.span())),
					}
				} else {
					Err(Error::option_duplicate(
						path.span(),
						&meta
							.path()
							.get_ident()
							.expect("unexpected skip syntax")
							.to_string(),
					))
				}
			}
			Meta::List(list) => {
				// Don't allow an empty list.
				if list.nested.is_empty() {
					return Err(Error::option_empty(list.span()));
				}

				// Get traits already set to be skipped.
				let traits = match self {
					// If no traits are set, change to empty `Skip::Traits` and return that.
					Skip::None => {
						*self = Skip::Traits(Vec::new());

						if let Skip::Traits(traits) = self {
							traits
						} else {
							unreachable!("unexpected variant")
						}
					}
					// If we are already skipping all traits, we can't skip again with constraints.
					Skip::All => return Err(Error::option_skip_all(list.span())),
					Skip::Traits(traits) => traits,
				};

				for nested_meta in &list.nested {
					if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
						let trait_ = Trait::from_path(path)?;

						// Don't allow unsupported traits to be skipped.
						if trait_.supports_skip() {
							// Don't allow to skip the same trait twice.
							if traits.contains(&trait_) {
								return Err(Error::option_skip_duplicate(
									path.span(),
									trait_.as_str(),
								));
							} else {
								// Don't allow to skip a trait already set to be skipped in the
								// parent.
								match skip_inner {
									Some(skip_inner) if skip_inner.skip(&trait_) => {
										return Err(Error::option_skip_inner(path.span()))
									}
									_ => {
										// Don't allow to skip trait that isn't being implemented.
										if derive_wheres.iter().any(|derive_where| {
											derive_where.trait_(&trait_).is_some()
										}) {
											traits.push(trait_)
										} else {
											return Err(Error::option_skip_trait(path.span()));
										}
									}
								}
							}
						} else {
							return Err(Error::option_skip_support(path.span(), trait_.as_str()));
						}
					} else {
						return Err(Error::option_syntax(nested_meta.span()));
					}
				}

				Ok(())
			}
			_ => Err(Error::option_syntax(meta.span())),
		}
	}

	/// Returns `true` if this item, variant or field is skipped with the given
	/// [`Trait`].
	pub fn skip(&self, trait_: &Trait) -> bool {
		match self {
			Skip::None => false,
			Skip::All => trait_.supports_skip(),
			Skip::Traits(traits) => {
				let skip = traits.contains(trait_);
				debug_assert!(!skip || trait_.supports_skip());
				skip
			}
		}
	}
}
