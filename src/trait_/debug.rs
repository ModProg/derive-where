//! [`Debug`](core::fmt::Debug) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Debug`](core::fmt::Debug).
pub struct Debug;

impl TraitImpl for Debug {
    fn as_str(&self) -> &'static str {
        "Debug"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Debug
    }

    fn supports_skip(&self) -> bool {
        true
    }
}
