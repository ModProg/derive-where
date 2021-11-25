//! [`PartialOrd`](core::cmp::PartialOrd) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`PartialOrd`](core::cmp::PartialOrd).
pub struct PartialOrd;

impl TraitImpl for PartialOrd {
    fn as_str(&self) -> &'static str {
        "PartialOrd"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::PartialOrd
    }

    fn supports_skip(&self) -> bool {
        true
    }
}
