//! [`Eq`](std::cmp::Eq) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Eq`](std::cmp::Eq).
pub struct Eq;

impl TraitImpl for Eq {
	fn as_str(&self) -> &'static str {
		"Eq"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::Eq
	}
}
