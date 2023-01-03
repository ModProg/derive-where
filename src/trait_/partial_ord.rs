//! [`PartialOrd`](std::cmp::PartialOrd) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use super::common_ord;
use crate::{Data, DeriveTrait, Item, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for
/// [`PartialOrd`](std::cmp::PartialOrd).
pub struct PartialOrd;

impl TraitImpl for PartialOrd {
	fn as_str(&self) -> &'static str {
		"PartialOrd"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::PartialOrd
	}

	fn build_signature(
		&self,
		item: &Item,
		_traits: &[DeriveTrait],
		trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		let body = common_ord::build_ord_signature(item, trait_, body);

		quote! {
			#[inline]
			fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
				#body
			}
		}
	}

	fn build_body(
		&self,
		_traits: &[DeriveTrait],
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		if data.is_empty(**trait_) || data.is_incomparable() {
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
