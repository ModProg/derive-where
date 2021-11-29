//! [`Ord`](core::cmp::Ord) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{DeriveTrait, Impl, TraitImpl};

use super::common_ord;

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Ord`](core::cmp::Ord).
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

    fn build_signature(&self, impl_: &Impl, body: &TokenStream) -> TokenStream {
        let body = common_ord::build_ord_signature(impl_, body);

        quote! {
            #[inline]
            fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                #body
            }
        }
    }
}
