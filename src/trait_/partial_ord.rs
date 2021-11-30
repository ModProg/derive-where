//! [`PartialOrd`](core::cmp::PartialOrd) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DeriveTrait, Impl, SimpleType, TraitImpl};

use super::common_ord;

/// Dummy-struct implement [`Trait`](crate::Trait) for [`PartialOrd`](core::cmp::PartialOrd).
pub struct PartialOrd;

impl TraitImpl for PartialOrd {
    fn as_str(&self) -> &'static str {
        "PartialOrd"
    }

    fn default_derive_trait(&self) -> DeriveTrait {
        DeriveTrait::PartialOrd
    }

    fn supports_skip(&self) -> bool {
        true
    }

    fn build_signature(&self, impl_: &Impl, body: &TokenStream) -> TokenStream {
        let body = common_ord::build_ord_signature(impl_, body);

        quote! {
            #[inline]
            fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                #body
            }
        }
    }

    fn build_body(&self, trait_: &DeriveTrait, data: &Data) -> TokenStream {
        if data.is_empty(trait_) {
            TokenStream::new()
        } else {
            match data.simple_type() {
                SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
                    let self_pattern = &fields.self_pattern;
                    let other_pattern = &fields.other_pattern;
                    let body = common_ord::build_ord_body(trait_, data);

                    quote! {
                        (#self_pattern, #other_pattern) => #body,
                    }
                }
                SimpleType::Unit(_) => TokenStream::new(),
                SimpleType::Union(_) => unreachable!("unexpected trait for union"),
            }
        }
    }
}
