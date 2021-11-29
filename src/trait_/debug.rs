//! [`Debug`](core::fmt::Debug) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{DeriveTrait, Impl, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Debug`](core::fmt::Debug).
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

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #body
                }
            }
        }
    }
}
