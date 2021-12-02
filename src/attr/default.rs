//! Attribute parsing for the `default` option.

use syn::{spanned::Spanned, Meta, Result};

use crate::{DeriveWhere, Error, Trait};

/// Stores if this variant should be the default when implementing
/// [`Default`](core::default::Default).
#[derive(Clone, Copy, Default)]
#[cfg_attr(test, derive(Debug))]
pub struct Default(pub bool);

impl Default {
	/// Token used for the `default` option.
	pub const DEFAULT: &'static str = "default";

	/// Adds a [`Meta`] to this [`Default`](self).
	pub fn add_attribute(
		&mut self,
		meta: &Meta,
		derive_wheres: &[DeriveWhere],
		accumulated_defaults: &mut Default,
	) -> Result<()> {
		debug_assert!(meta.path().is_ident(Self::DEFAULT));

		if let Meta::Path(path) = meta {
			if self.0 {
				Err(Error::option_duplicate(path.span(), Self::DEFAULT))
			} else {
				let mut impl_default = false;

				for derive_where in derive_wheres {
					if derive_where.trait_(Trait::Default).is_some() {
						impl_default = true;
						break;
					}
				}

				if impl_default {
					if accumulated_defaults.0 {
						Err(Error::default_duplicate(path.span()))
					} else {
						accumulated_defaults.0 = true;
						self.0 = true;
						Ok(())
					}
				} else {
					Err(Error::default(path.span()))
				}
			}
		} else {
			Err(Error::option_syntax(meta.span()))
		}
	}
}
