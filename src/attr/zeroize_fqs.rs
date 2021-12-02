//! Attribute parsing for the `Zeroize(fqs)` option.

use syn::{spanned::Spanned, Meta, NestedMeta, Result};

use crate::{Error, Trait, TraitImpl};

/// Stores if this field should use FQS to call [`Zeroize::zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html#tymethod.zeroize).
#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct ZeroizeFqs(pub bool);

impl ZeroizeFqs {
	/// Token used for the `Zeroize(fqs)` option.
	const FQS: &'static str = "fqs";

	/// Adds a [`Meta`] to this [`ZeroizeFqs`].
	pub fn add_attribute(&mut self, meta: &Meta) -> Result<()> {
		debug_assert!(meta.path().is_ident(Trait::Zeroize.as_str()));

		// TODO: don't allow `Zeroize(fqs)` if `Zeroize` isn't being implemented

		match meta {
			Meta::List(list) => {
				for nested_meta in &list.nested {
					match nested_meta {
						NestedMeta::Meta(Meta::Path(path)) => {
							if path.is_ident(Self::FQS) {
								if self.0 {
									return Err(Error::option_duplicate(path.span(), Self::FQS));
								} else {
									self.0 = true
								}
							} else {
								return Err(Error::option(path.span()));
							}
						}
						_ => return Err(Error::option_syntax(nested_meta.span())),
					}
				}

				Ok(())
			}
			Meta::Path(path) => Err(Error::option_required(path.span(), Trait::Zeroize.as_str())),
			_ => Err(Error::option_syntax(meta.span())),
		}
	}
}
