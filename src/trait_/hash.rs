//! [`Hash`](core::hash::Hash) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{DeriveTrait, Impl, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Hash`](core::hash::Hash).
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

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                match self {
                    #body
                }
            }
        }
    }
}
