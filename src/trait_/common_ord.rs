//! Common implementation help for [`PartialOrd`] and [`Ord`].

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DeriveTrait, Item};

/// Build signature for [`PartialOrd`] and [`Ord`].
pub fn build_ord_signature(item: &Item, trait_: &DeriveTrait, body: &TokenStream) -> TokenStream {
	use DeriveTrait::*;

	let mut equal = quote! { ::core::cmp::Ordering::Equal };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	if let PartialOrd = trait_ {
		equal = quote! { ::core::option::Option::Some(#equal) };
	}

	match item {
		// If there is more than one variant, check for the discriminant.
		Item::Enum { variants, .. } if variants.len() > 1 => {
			// If all variants are empty, return `Equal`.
			let body = if item.is_empty(trait_) {
				quote! { #equal }
			}
			// Compare variant data and return `Equal` in the rest pattern if there are any empty
			// variants.
			else if variants.iter().any(|variant| variant.is_empty(trait_)) {
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

			// Nightly or unsafe (default) implementation.
			#[cfg(any(feature = "nightly", not(feature = "safe")))]
			{
				let path = trait_.path();
				let method = match trait_ {
					PartialOrd => quote! { partial_cmp },
					Ord => quote! { cmp },
					_ => unreachable!("unsupported trait in `prepare_ord`"),
				};

				// Nightly implementation.
				#[cfg(feature = "nightly")]
				quote! {
					let __self_disc = ::core::intrinsics::discriminant_value(self);
					let __other_disc = ::core::intrinsics::discriminant_value(__other);

					if __self_disc == __other_disc {
						#body
					} else {
						#path::#method(&__self_disc, &__other_disc)
					}
				}
				// Unsafe (default) implementation.
				#[cfg(not(feature = "nightly"))]
				quote! {
					let __self_disc = ::core::mem::discriminant(self);
					let __other_disc = ::core::mem::discriminant(__other);

					if __self_disc == __other_disc {
						#body
					} else {
						#path::#method(
							&unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
							&unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
						)
					}
				}
			}
			// Safe implementation when not on nightly.
			#[cfg(all(not(feature = "nightly"), feature = "safe"))]
			{
				let mut less = quote! { ::core::cmp::Ordering::Less };
				let mut greater = quote! { ::core::cmp::Ordering::Greater };

				// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
				if let PartialOrd = trait_ {
					less = quote! { ::core::option::Option::Some(#less) };
					greater = quote! { ::core::option::Option::Some(#greater) };
				}

				let mut different = Vec::with_capacity(variants.len());

				// Build separate `match` arms to compare different variants to each
				// other. The index for these variants is used to determine which
				// `Ordering` to return.
				for (index, variant) in variants.iter().enumerate() {
					let mut arms = Vec::with_capacity(variants.len() - 1);

					for (index_other, variant_other) in variants.iter().enumerate() {
						// Make sure we aren't comparing the same variant with itself.
						if index != index_other {
							use std::cmp::Ordering::*;

							let ordering = match index.cmp(&index_other) {
								Less => &less,
								Equal => &equal,
								Greater => &greater,
							};

							let pattern = &variant_other.other_pattern();

							arms.push(quote! {
								#pattern => #ordering,
							});
						}
					}

					let pattern = &variant.self_pattern();

					different.push(quote! {
						#pattern => match __other {
							#(#arms)*
							_ => ::core::unreachable!("comparing variants yielded unexpected results"),
						},
					});
				}

				quote! {
					let __self_disc = ::core::mem::discriminant(self);
					let __other_disc = ::core::mem::discriminant(__other);

					if __self_disc == __other_disc {
						#body
					} else {
						match self {
							#(#different)*
						}
					}
				}
			}
		}
		// If there is only one variant and it's empty or if the struct is empty, simple
		// return `Equal`.
		item if item.is_empty(trait_) => {
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
		.iter_self_ident(trait_)
		.rev()
		.zip(data.iter_other_ident(trait_).rev())
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
