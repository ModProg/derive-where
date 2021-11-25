//! [`Copy`](core::marker::Copy) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Copy`](core::marker::Copy).
pub struct Copy;

impl TraitImpl for Copy {
    fn as_str(&self) -> &'static str {
        "Copy"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Copy
    }
}
