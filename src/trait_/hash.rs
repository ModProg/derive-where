//! [`Hash`](core::hash::Hash) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Hash`](core::hash::Hash).
pub struct Hash;

impl TraitImpl for Hash {
    fn as_str(&self) -> &'static str {
        "Hash"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Hash
    }

    fn supports_skip(&self) -> bool {
        true
    }
}
