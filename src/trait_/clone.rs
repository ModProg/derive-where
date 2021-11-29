//! [`Clone`](core::clone::Clone) implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{TraitBound, TraitBoundModifier, TypeParamBound};

use crate::{DeriveTrait, Impl, Item, Trait, TraitImpl};

/// Dummy-struct implement [`Trait`] for [`Clone`](core::clone::Clone).
pub struct Clone;

impl TraitImpl for Clone {
    fn as_str(&self) -> &'static str {
        "Clone"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Clone
    }

    fn additional_where_bounds(&self, data: &Item) -> Option<TypeParamBound> {
        // `Clone` for unions requires the `Copy` bound.
        if let Item::Union(..) = data {
            Some(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: Trait::Copy.default_derive_trait().path(),
            }))
        } else {
            None
        }
    }

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            #[inline]
            fn clone(&self) -> Self {
                match self {
                    #body
                }
            }
        }
    }
}
