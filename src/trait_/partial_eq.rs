//! [`PartialEq`](core::cmp::PartialEq) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`PartialEq`](core::cmp::PartialEq).
pub struct PartialEq;

impl TraitImpl for PartialEq {
    fn as_str(&self) -> &'static str {
        "PartialEq"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::PartialEq
    }

    fn supports_skip(&self) -> bool {
        true
    }
}
