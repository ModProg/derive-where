//! [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Lit, Meta, MetaList, NestedMeta, Path, Result};

use crate::{util, Data, DeriveTrait, Error, Item, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) .
pub struct Zeroize;

impl TraitImpl for Zeroize {
	fn as_str(&self) -> &'static str {
		"Zeroize"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::Zeroize { crate_: None }
	}

	fn parse_derive_trait(&self, list: MetaList) -> Result<DeriveTrait> {
		// This is already checked in `DeriveTrait::from_stream`.
		debug_assert!(!list.nested.is_empty());

		let mut crate_ = None;

		for nested_meta in list.nested {
			match &nested_meta {
				NestedMeta::Meta(Meta::Path(path)) => {
					if path.is_ident("drop") {
						return Err(Error::deprecated_zeroize_drop(path.span()));
					} else {
						return Err(Error::option_trait(path.span(), self.as_str()));
					}
				}
				NestedMeta::Meta(Meta::NameValue(name_value)) => {
					if name_value.path.is_ident("crate") {
						// Check for duplicate `crate` option.
						if crate_.is_none() {
							if let Lit::Str(lit_str) = &name_value.lit {
								match lit_str.parse::<Path>() {
									Ok(path) => {
										if path == util::path_from_strs(&["zeroize"]) {
											return Err(Error::path_unnecessary(
												path.span(),
												"::zeroize",
											));
										}

										crate_ = Some(path);
									}
									Err(error) => return Err(Error::path(lit_str.span(), error)),
								}
							} else {
								return Err(Error::option_syntax(name_value.lit.span()));
							}
						} else {
							return Err(Error::option_duplicate(name_value.span(), "crate"));
						}
					} else {
						return Err(Error::option_trait(name_value.path.span(), self.as_str()));
					}
				}
				_ => {
					return Err(Error::option_syntax(nested_meta.span()));
				}
			}
		}

		Ok(DeriveTrait::Zeroize { crate_ })
	}

	fn supports_skip(&self) -> bool {
		true
	}

	fn build_signature(
		&self,
		item: &Item,
		_traits: &[DeriveTrait],
		trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		match item {
			Item::Item(data) if data.is_empty(trait_) => quote! {
				fn zeroize(&mut self) { }
			},
			_ => {
				let trait_path = trait_.path();
				quote! {
					fn zeroize(&mut self) {
						use #trait_path;

						match self {
							#body
						}
					}
				}
			}
		}
	}

	fn build_body(
		&self,
		_traits: &[DeriveTrait],
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		if data.is_empty(trait_) {
			TokenStream::new()
		} else {
			match data.simple_type() {
				SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
					let trait_path = trait_.path();
					let self_pattern = fields.self_pattern_mut();

					let body = data
						.iter_fields(trait_)
						.zip(data.iter_self_ident(trait_))
						.map(|(field, self_ident)| {
							if field.attr.zeroize_fqs.0 {
								quote! { #trait_path::zeroize(#self_ident); }
							} else {
								quote! { #self_ident.zeroize(); }
							}
						});

					quote! {
						#self_pattern => {
							#(#body)*
						}
					}
				}
				SimpleType::Unit(_) => TokenStream::new(),
				SimpleType::Union(_) => unreachable!("unexpected trait for union"),
			}
		}
	}
}
