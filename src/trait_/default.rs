//! [`Default`](core::default::Default) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{DeriveTrait, Impl, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Default`](core::default::Default).
pub struct Default;

impl TraitImpl for Default {
    fn as_str(&self) -> &'static str {
        "Default"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::Default
    }

    fn build_signature(&self, _impl_: &Impl, body: &TokenStream) -> TokenStream {
        quote! {
            fn default() -> Self {
                #body
            }
        }
    }
}
