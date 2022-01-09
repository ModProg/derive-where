//! [`ZeroizeOnDrop`](https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeOnDrop.html) implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Lit, Meta, MetaList, NestedMeta, Path, Result};

use crate::{util, Data, DeriveTrait, Error, Item, SimpleType, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`ZeroizeOnDrop`](https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeOnDrop.html) .
pub struct ZeroizeOnDrop;

impl TraitImpl for ZeroizeOnDrop {
	fn as_str(&self) -> &'static str {
		"ZeroizeOnDrop"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::ZeroizeOnDrop { crate_: None }
	}

	fn parse_derive_trait(&self, list: MetaList) -> Result<DeriveTrait> {
		// This is already checked in `DeriveTrait::from_stream`.
		debug_assert!(!list.nested.is_empty());

		let mut crate_ = None;

		for nested_meta in list.nested {
			match &nested_meta {
				NestedMeta::Meta(Meta::Path(path)) => {
					return Err(Error::option_trait(path.span(), self.as_str()))
				}
				NestedMeta::Meta(Meta::NameValue(name_value)) => {
					if name_value.path.is_ident("crate") {
						// Check for duplicate `crate` option.
						if crate_.is_none() {
							if let Lit::Str(lit_str) = &name_value.lit {
								match lit_str.parse() {
									Ok(path) => {
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

		Ok(DeriveTrait::ZeroizeOnDrop { crate_ })
	}

	fn supports_skip(&self) -> bool {
		true
	}

	#[allow(unused_variables)]
	fn additional_impl(&self, trait_: &DeriveTrait) -> Option<(Path, TokenStream)> {
		#[cfg(feature = "zeroize-on-drop")]
		return Some((trait_.path(), quote! {}));
		#[cfg(not(feature = "zeroize-on-drop"))]
		None
	}

	fn impl_path(&self, _trait_: &DeriveTrait) -> Path {
		util::path_from_strs(&["core", "ops", "Drop"])
	}

	fn build_signature(
		&self,
		item: &Item,
		trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		match item {
			Item::Item(data) if data.is_empty(trait_) => quote! {
				fn drop(&mut self) { }
			},
			_ => {
				#[cfg(feature = "zeroize-on-drop")]
				{
					let crate_ = trait_.crate_();
					let internal = util::path_segment("__internal");

					let mut assert_zeroize = crate_.clone();
					assert_zeroize
						.segments
						.extend([internal.clone(), util::path_segment("AssertZeroize")]);

					let mut assert_zeroize_on_drop = crate_;
					assert_zeroize_on_drop
						.segments
						.extend([internal, util::path_segment("AssertZeroizeOnDrop")]);

					quote! {
						fn drop(&mut self) {
							use #assert_zeroize;
							use #assert_zeroize_on_drop;

							match self {
								#body
							}
						}
					}
				}
				#[cfg(not(feature = "zeroize-on-drop"))]
				quote! {
					fn drop(&mut self) {
						#body
					}
				}
			}
		}
	}

	fn build_body(&self, trait_: &DeriveTrait, data: &Data) -> TokenStream {
		if data.is_empty(trait_) {
			TokenStream::new()
		} else {
			match data.simple_type() {
				SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
					#[cfg(feature = "zeroize-on-drop")]
					{
						let self_pattern = fields.self_pattern_mut();
						let self_ident = data.iter_self_ident(trait_);

						quote! {
							#self_pattern => {
								#(#self_ident.zeroize_or_on_drop();)*
							}
						}
					}
					#[cfg(not(feature = "zeroize-on-drop"))]
					{
						// Use unused variables.
						let _ = fields;

						let path = util::path_from_root_and_strs(trait_.crate_(), &["Zeroize"]);

						quote! {
							#path::zeroize(self);
						}
					}
				}
				SimpleType::Unit(_) => TokenStream::new(),
				SimpleType::Union(_) => unreachable!("unexpected trait for union"),
			}
		}
	}
}
