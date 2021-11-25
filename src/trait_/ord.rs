//! [`Ord`](core::cmp::Ord) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Ord`](core::cmp::Ord).
pub struct Ord;

impl TraitImpl for Ord {
    fn as_str(&self) -> &'static str {
        "Ord"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Ord
    }

    fn supports_skip(&self) -> bool {
        true
    }
}
