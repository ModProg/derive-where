//! [`Clone`](core::clone::Clone) implementation.

use syn::{TraitBound, TraitBoundModifier, TypeParamBound};

use crate::{Data, DeriveTrait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Clone`](core::clone::Clone).
pub struct Clone;

impl TraitImpl for Clone {
    fn as_str(&self) -> &'static str {
        "Clone"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Clone
    }

    fn additional_where_bounds(&self, data: &Data) -> Option<TypeParamBound> {
        // `Clone` for unions requires the `Copy` bound.
        if let Data::Union(..) = data {
            Some(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: self.default_derive_trait().path(),
            }))
        } else {
            None
        }
    }
}
