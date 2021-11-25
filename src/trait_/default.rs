//! [`Default`](core::default::Default) implementation.

use crate::{DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Default`](core::default::Default).
pub struct Default;

impl TraitImpl for Default {
    fn as_str(&self) -> &'static str {
        "Default"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Default
    }
}
