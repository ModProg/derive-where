//! Common implementation help for [`PartialOrd`] and [`Ord`].

use proc_macro2::TokenStream;
use quote::quote;
#[cfg(not(feature = "nightly"))]
use syn::PatOr;

#[cfg(not(feature = "nightly"))]
use crate::Discriminant;
use crate::{Data, DeriveTrait, Item, SimpleType, Trait};

/// Build signature for [`PartialOrd`] and [`Ord`].
pub fn build_ord_signature(
	item: &Item,
	traits: &[DeriveTrait],
	trait_: &DeriveTrait,
	body: &TokenStream,
) -> TokenStream {
	let mut equal = quote! { ::core::cmp::Ordering::Equal };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	if let DeriveTrait::PartialOrd = trait_ {
		equal = quote! { ::core::option::Option::Some(#equal) };
	}

	match item {
		// If the item is incomparable return `None`
		item if item.is_incomparable() => {
			quote! { ::core::option::Option::None }
		}
		// If there is more than one variant, check for the discriminant.
		Item::Enum {
			#[cfg(not(feature = "nightly"))]
			discriminant,
			variants,
			..
		} if variants.len() > 1 => {
			// In case the discriminant matches:
			// If all variants are empty, return `Equal`.
			let body_equal = if item.is_empty(**trait_) {
				quote! { #equal }
			}
			// Compare variant data and return `Equal` in the rest pattern if there are any empty
			// variants that are comparable.
			else if variants
				.iter()
				.any(|variant| variant.is_empty(**trait_) && !variant.is_incomparable())
			{
				quote! {
					match (self, __other) {
						#body
						_ => #equal,
					}
				}
			}
			// Insert `unreachable!` in the rest pattern if no variants are empty.
			else {
				#[cfg(not(feature = "safe"))]
				// This follows the standard implementation.
				let rest = quote! { unsafe { ::core::hint::unreachable_unchecked() } };
				#[cfg(feature = "safe")]
				let rest = quote! { ::core::unreachable!("comparing variants yielded unexpected results") };

				quote! {
					match (self, __other) {
						#body
						_ => #rest,
					}
				}
			};

			let incomparable = build_incomparable_pattern(variants);

			// If there is only one comparable variant, it has to be it when it is non
			// incomparable.
			let mut comparable = variants.iter().filter(|v| !v.is_incomparable());
			// Takes the first value from the iterator, but only when there is only one
			// (second yields none).
			if let (Some(comparable), None) = (comparable.next(), comparable.next()) {
				let incomparable = incomparable.expect("there should be > 1 variants");
				// Either compare the single variant or return `Equal` when it is empty
				let equal = if comparable.is_empty(**trait_) {
					equal
				} else {
					body_equal
				};
				quote! {
					if ::core::matches!(self, #incomparable) || ::core::matches!(__other, #incomparable) {
						::core::option::Option::None
					} else {
						#equal
					}
				}
			} else {
				let incomparable = incomparable.into_iter();
				let incomparable = quote! {
					#(if ::core::matches!(self, #incomparable) || ::core::matches!(__other, #incomparable) {
						return ::core::option::Option::None;
					})*
				};

				let path = trait_.path();
				let method = match trait_ {
					DeriveTrait::PartialOrd => quote! { partial_cmp },
					DeriveTrait::Ord => quote! { cmp },
					_ => unreachable!("unsupported trait in `prepare_ord`"),
				};

				// Nightly implementation.
				#[cfg(feature = "nightly")]
				quote! {
					#incomparable

					let __self_disc = ::core::intrinsics::discriminant_value(self);
					let __other_disc = ::core::intrinsics::discriminant_value(__other);

					if __self_disc == __other_disc {
						#body_equal
					} else {
						#path::#method(&__self_disc, &__other_disc)
					}
				}

				#[cfg(not(feature = "nightly"))]
				{
					let body_else = match discriminant {
						Discriminant::Single => {
							unreachable!("we should only generate this code with multiple variants")
						}
						Discriminant::UnitDefault => {
							if traits.iter().any(|trait_| trait_ == Trait::Copy) {
								quote! {
									#path::#method(
										*self as isize,
										*__other as isize,
									)
								}
							} else if traits.iter().any(|trait_| trait_ == Trait::Clone) {
								quote! {
									#path::#method(
										self.clone() as isize,
										__other.clone() as isize,
									)
								}
							} else {
								build_recursive_order(trait_, variants, &incomparable)
							}
						}
						Discriminant::Unknown => {
							build_recursive_order(trait_, variants, &incomparable)
						}
						#[cfg(feature = "safe")]
						Discriminant::UnitRepr(repr) => {
							if traits.iter().any(|trait_| trait_ == Trait::Copy) {
								quote! {
									#path::#method(
										*self as #repr,
										*__other as #repr,
									)
								}
							} else if traits.iter().any(|trait_| trait_ == Trait::Clone) {
								quote! {
									#path::#method(
										self.clone() as #repr,
										__other.clone() as #repr,
									)
								}
							} else {
								build_recursive_order(trait_, variants, &incomparable)
							}
						}
						#[cfg(not(feature = "safe"))]
						Discriminant::UnitRepr(repr) | Discriminant::Repr(repr) => {
							quote! {
								#path::#method(
									unsafe { *<*const _>::from(self).cast::<#repr>() },
									unsafe { *<*const _>::from(__other).cast::<#repr>() },
								)
							}
						}
						#[cfg(feature = "safe")]
						Discriminant::Repr(_) => build_recursive_order(trait_, variants, &incomparable),
					};

					quote! {
						#incomparable

						let __self_disc = ::core::mem::discriminant(self);
						let __other_disc = ::core::mem::discriminant(__other);

						if __self_disc == __other_disc {
							#body_equal
						} else {
							#body_else
						}
					}
				}
			}
		}
		// If there is only one variant and it's empty or if the struct is empty, simply
		// return `Equal`.
		item if item.is_empty(**trait_) => {
			quote! { #equal }
		}
		_ => {
			quote! {
				match (self, __other) {
					#body
				}
			}
		}
	}
}

/// Builds order comparison recursively for all variants.
#[cfg(not(feature = "nightly"))]
fn build_recursive_order(
	trait_: &DeriveTrait,
	variants: &[Data<'_>],
	incomparable: &TokenStream,
) -> TokenStream {
	let mut less = quote! { ::core::cmp::Ordering::Less };
	let mut greater = quote! { ::core::cmp::Ordering::Greater };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	if let DeriveTrait::PartialOrd = trait_ {
		less = quote! { ::core::option::Option::Some(#less) };
		greater = quote! { ::core::option::Option::Some(#greater) };
	}

	let mut different = Vec::with_capacity(variants.len());
	let variants: Vec<_> = variants.iter().filter(|v| !v.is_incomparable()).collect();

	// Build separate `match` arms to compare different variants to each
	// other. The index for these variants is used to determine which
	// `Ordering` to return.
	for (index, variant) in variants.iter().enumerate() {
		let pattern = &variant.self_pattern();

		// The first variant is always `Less` then everything.
		if index == 0 {
			different.push(quote! {
				#pattern => #less,
			})
		}
		// The last variant is always `Greater` then everything.
		else if index == variants.len() - 1 {
			different.push(quote! {
				#pattern => #greater,
			})
		}
		// Any variant between the first and last.
		else {
			// Collect all variants that are `Less`.
			let cases = variants
				.iter()
				.enumerate()
				.filter(|(index_other, _)| *index_other < index)
				.map(|(_, variant_other)| variant_other.other_pattern_skip().clone())
				.collect();

			// Build one match arm pattern with all variants that are `Greater`.
			let pattern_less = PatOr {
				attrs: Vec::new(),
				leading_vert: None,
				cases,
			};

			// All other variants are `Less`.
			different.push(quote! {
				#pattern => match __other {
					#pattern_less => #greater,
					_ => #less,
				},
			});
		}
	}

	let rest = if incomparable.is_empty() {
		quote!()
	} else {
		quote!(_ => unreachable!("incomparable variants should have already returned"),)
	};

	quote! {
		match self {
			#(#different)*
			#rest
		}
	}
}

/// Build `match` arms for [`PartialOrd`] and [`Ord`].
pub fn build_ord_body(trait_: &DeriveTrait, data: &Data) -> TokenStream {
	use DeriveTrait::*;

	let path = trait_.path();
	let mut equal = quote! { ::core::cmp::Ordering::Equal };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	let method = match trait_ {
		PartialOrd => {
			equal = quote! { ::core::option::Option::Some(#equal) };
			quote! { partial_cmp }
		}
		Ord => quote! { cmp },
		_ => unreachable!("unsupported trait in `build_ord`"),
	};

	// The match arm starts with `Ordering::Equal`. This will become the
	// whole `match` arm if no fields are present.
	let mut body = quote! { #equal };

	// Builds `match` arms backwards, using the `match` arm of the field coming
	// afterwards. `rev` has to be called twice separately because it can't be
	// called on `zip`
	for (field_temp, field_other) in data
		.iter_self_ident(**trait_)
		.rev()
		.zip(data.iter_other_ident(**trait_).rev())
	{
		body = quote! {
			match #path::#method(#field_temp, #field_other) {
				#equal => #body,
				__cmp => __cmp,
			}
		};
	}

	body
}

/// Generate a match arm that returns `body` for all incomparable `variants`
pub fn build_incomparable_pattern(variants: &[Data]) -> Option<TokenStream> {
	let mut incomparable = variants
		.iter()
		.filter(|variant| variant.is_incomparable())
		.map(|variant @ Data { path, .. }| match variant.simple_type() {
			SimpleType::Struct(_) => quote!(#path{..}),
			SimpleType::Tuple(_) => quote!(#path(..)),
			SimpleType::Union(_) => unreachable!("enum variants cannot be unions"),
			SimpleType::Unit(_) => quote!(#path),
		})
		.peekable();
	if incomparable.peek().is_some() {
		Some(quote! {
			#(#incomparable)|*
		})
	} else {
		None
	}
}
