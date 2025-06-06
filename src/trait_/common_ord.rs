//! Common implementation help for [`PartialOrd`] and [`Ord`].

#[cfg(not(feature = "nightly"))]
use std::borrow::Cow;

use proc_macro2::TokenStream;
#[cfg(not(feature = "nightly"))]
use proc_macro2::{Literal, Span};
use quote::quote;
#[cfg(not(feature = "nightly"))]
use syn::{parse_quote, Expr, ExprLit, LitInt, Path};

#[cfg(not(feature = "nightly"))]
use crate::{item::Representation, DeriveTrait, Discriminant};
use crate::{Data, DeriveWhere, Item, SimpleType, SplitGenerics, Trait, TraitImpl};

/// Build signature for [`PartialOrd`] and [`Ord`].
pub fn build_ord_signature(
	item: &Item,
	#[cfg_attr(feature = "nightly", allow(unused_variables))] generics: &SplitGenerics<'_>,
	#[cfg_attr(feature = "nightly", allow(unused_variables))] derive_where: &DeriveWhere,
	trait_: &dyn TraitImpl,
	body: &TokenStream,
) -> TokenStream {
	let mut equal = quote! { ::core::cmp::Ordering::Equal };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	if let Trait::PartialOrd = **trait_ {
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
				None
			}
			// Compare variant data and return `Equal` in the rest pattern if there are any empty
			// variants that are comparable.
			else if variants
				.iter()
				.any(|variant| variant.is_empty(**trait_) && !variant.is_incomparable())
			{
				Some(quote! {
					match (self, __other) {
						#body
						_ => #equal,
					}
				})
			}
			// Insert `unreachable!` in the rest pattern if no variants are empty.
			else {
				#[cfg(not(feature = "safe"))]
				// This follows the standard implementation.
				let rest = quote! { unsafe { ::core::hint::unreachable_unchecked() } };
				#[cfg(feature = "safe")]
				let rest = quote! { ::core::unreachable!("comparing variants yielded unexpected results") };

				Some(quote! {
					match (self, __other) {
						#body
						_ => #rest,
					}
				})
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
					body_equal.unwrap_or(equal)
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
				let method = match **trait_ {
					Trait::PartialOrd => quote! { partial_cmp },
					Trait::Ord => quote! { cmp },
					_ => unreachable!("unsupported trait in `prepare_ord`"),
				};

				// Nightly implementation.
				#[cfg(feature = "nightly")]
				if let Some(body_equal) = body_equal {
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
				} else {
					quote! {
						#incomparable

						#path::#method(
							&::core::intrinsics::discriminant_value(self),
							&::core::intrinsics::discriminant_value(__other),
						)
					}
				}

				#[cfg(not(feature = "nightly"))]
				{
					let body_else = match discriminant {
						Discriminant::Single => {
							unreachable!("we should only generate this code with multiple variants")
						}
						Discriminant::Unit => {
							if derive_where.contains(Trait::Copy) {
								quote! {
									#path::#method(&(*self as isize), &(*__other as isize))
								}
							} else if derive_where.contains(Trait::Clone) {
								let clone = DeriveTrait::Clone.path();
								quote! {
									#path::#method(&(#clone::clone(self) as isize), &(#clone::clone(__other) as isize))
								}
							} else {
								build_discriminant_order(
									None, item, generics, variants, &path, &method,
								)
							}
						}
						Discriminant::Data => {
							build_discriminant_order(None, item, generics, variants, &path, &method)
						}
						Discriminant::UnitRepr(repr) => {
							if derive_where.contains(Trait::Copy) {
								quote! {
									#path::#method(&(*self as #repr), &(*__other as #repr))
								}
							} else if derive_where.contains(Trait::Clone) {
								let clone = DeriveTrait::Clone.path();
								quote! {
									#path::#method(&(#clone::clone(self) as #repr), &(#clone::clone(__other) as #repr))
								}
							} else {
								#[cfg(feature = "safe")]
								let body_else = {
									build_discriminant_order(
										Some(*repr),
										item,
										generics,
										variants,
										&path,
										&method,
									)
								};
								#[cfg(not(feature = "safe"))]
								let body_else = quote! {
									#path::#method(
										&unsafe { *<*const _>::from(self).cast::<#repr>() },
										&unsafe { *<*const _>::from(__other).cast::<#repr>() },
									)
								};

								body_else
							}
						}
						#[cfg(not(feature = "safe"))]
						Discriminant::DataRepr(repr) => {
							quote! {
								#path::#method(
									&unsafe { *<*const _>::from(self).cast::<#repr>() },
									&unsafe { *<*const _>::from(__other).cast::<#repr>() },
								)
							}
						}
						#[cfg(feature = "safe")]
						Discriminant::DataRepr(repr) => build_discriminant_order(
							Some(*repr),
							item,
							generics,
							variants,
							&path,
							&method,
						),
					};

					if let Some(body_equal) = body_equal {
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
					} else {
						quote! {
							#incomparable

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

/// Create `discriminant()` function and use it to do the comparison.
#[cfg(not(feature = "nightly"))]
fn build_discriminant_order(
	repr: Option<Representation>,
	item: &Item,
	generics: &SplitGenerics<'_>,
	variants: &[Data<'_>],
	path: &Path,
	method: &TokenStream,
) -> TokenStream {
	let mut discriminants = Vec::<Cow<Expr>>::with_capacity(variants.len());
	let mut last_expression: Option<(Option<usize>, usize)> = None;

	for variant in variants {
		let discriminant = if let Some(discriminant) = variant.discriminant {
			last_expression = Some((Some(discriminants.len()), 0));
			Cow::Borrowed(discriminant)
		} else {
			let discriminant = match &mut last_expression {
				Some((Some(expr_index), counter)) => {
					let expr = &discriminants[*expr_index];
					*counter += 1;
					let counter = Literal::usize_unsuffixed(*counter);
					parse_quote! { (#expr) + #counter }
				}
				Some((None, counter)) => {
					*counter += 1;

					ExprLit {
						attrs: Vec::new(),
						lit: LitInt::new(&counter.to_string(), Span::call_site()).into(),
					}
					.into()
				}
				None => {
					last_expression = Some((None, 0));
					ExprLit {
						attrs: Vec::new(),
						lit: LitInt::new("0", Span::call_site()).into(),
					}
					.into()
				}
			};

			Cow::Owned(discriminant)
		};

		discriminants.push(discriminant);
	}

	let variants = variants
		.iter()
		.zip(discriminants)
		.map(|(variant, discriminant)| {
			let pattern = variant.self_pattern();

			quote! {
				#pattern => #discriminant
			}
		});

	// `isize` is currently used by Rust as the default representation when none is
	// defined.
	let repr = repr.unwrap_or(Representation::ISize).to_token();

	let item = item.ident();
	let SplitGenerics {
		imp,
		ty,
		where_clause,
	} = generics;

	quote! {
		const fn __discriminant #imp(__this: &#item #ty) -> #repr #where_clause {
			match __this {
				#(#variants),*
			}
		}

		#path::#method(&__discriminant(self), &__discriminant(__other))
	}
}

/// Build `match` arms for [`PartialOrd`] and [`Ord`].
pub fn build_ord_body(trait_: &dyn TraitImpl, data: &Data) -> TokenStream {
	let path = trait_.path();
	let mut equal = quote! { ::core::cmp::Ordering::Equal };

	// Add `Option` to `Ordering` if we are implementing `PartialOrd`.
	let method = match **trait_ {
		Trait::PartialOrd => {
			equal = quote! { ::core::option::Option::Some(#equal) };
			quote! { partial_cmp }
		}
		Trait::Ord => quote! { cmp },
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
			SimpleType::Union => unreachable!("enum variants cannot be unions"),
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
