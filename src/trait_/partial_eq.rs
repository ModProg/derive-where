//! [`PartialEq`](std::cmp::PartialEq) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DeriveTrait, Item, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for
/// [`PartialEq`](std::cmp::PartialEq).
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

	fn build_signature(
		&self,
		item: &Item,
		trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		let body = {
			match item {
				// If there is more than one variant and not all variants are empty, check for
				// discriminant and match on variant data.
				Item::Enum { variants, .. } if variants.len() > 1 && !item.is_empty(trait_) => {
					// Return `true` in the rest pattern if there are any empty variants.
					let rest = if variants.iter().any(|variant| variant.is_empty(trait_)) {
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
				// If there is more than one variant and all are empty, check for discriminant and
				// simply return `true`.
				Item::Enum { variants, .. } if variants.len() > 1 && item.is_empty(trait_) => {
					quote! {
						if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
							true
						} else {
							false
						}
					}
				}
				// If there is only one variant and it's empty or if the struct is empty, simple
				// return `true`.
				item if item.is_empty(trait_) => {
					quote! { true }
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

	fn build_body(&self, trait_: &DeriveTrait, data: &Data) -> TokenStream {
		if data.is_empty(trait_) {
			TokenStream::new()
		} else {
			match data.simple_type() {
				SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
					let self_pattern = &fields.self_pattern;
					let other_pattern = &fields.other_pattern;
					let trait_path = trait_.path();
					let self_ident = data.iter_self_ident(trait_);
					let other_ident = data.iter_other_ident(trait_);

					quote! {
						(#self_pattern, #other_pattern) =>
							true #(&& #trait_path::eq(#self_ident, #other_ident))*,
					}
				}
				SimpleType::Unit(_) => TokenStream::new(),
				SimpleType::Union(_) => unreachable!("unexpected trait for union"),
			}
		}
	}
}
