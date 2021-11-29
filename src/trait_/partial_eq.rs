//! [`PartialEq`](core::cmp::PartialEq) implementation.

use crate::{util, DeriveTrait, Impl, Item, TraitImpl};
use proc_macro2::TokenStream;
use quote::quote;

/// Dummy-struct implement [`Trait`](crate::Trait) for [`PartialEq`](core::cmp::PartialEq).
pub struct PartialEq;

impl TraitImpl for PartialEq {
    fn as_str(&self) -> &'static str {
        "PartialEq"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::PartialEq
    }

    fn supports_skip(&self) -> bool {
        true
    }

    fn build_signature(&self, impl_: &Impl, body: &TokenStream) -> TokenStream {
        let body = {
            match &impl_.input.item {
                // Only check for discriminators if there is more than one variant.
                Item::Enum { variants, .. } if variants.len() > 1 => {
                    // If there are any unit variants, skip comparing them and instead return `true`. Otherwise panic as it should be unreachable.
                    let rest = if util::unit_found(&impl_.input.item) {
                        quote! { true }
                    } else {
                        #[cfg(not(feature = "safe"))]
                        // This follows the standard implementation.
                        quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                        #[cfg(feature = "safe")]
                        quote! { ::core::unreachable!("comparing variants yielded unexpected results") }
                    };

                    quote! {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                #body
                                _ => #rest,
                            }
                        } else {
                            false
                        }
                    }
                }
                // If only one variant was found and it's a unit variant, return `true`.
                Item::Enum { variants, .. }
                    if variants.len() == 1 && util::unit_found(&impl_.input.item) =>
                {
                    quote! {
                        true
                    }
                }
                _ => {
                    quote! {
                        match (self, __other) {
                            #body
                        }
                    }
                }
            }
        };

        quote! {
            #[inline]
            fn eq(&self, __other: &Self) -> bool {
                #body
            }
        }
    }
}
