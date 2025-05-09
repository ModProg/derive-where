//! [`ZeroizeOnDrop`](https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeOnDrop.html) implementation.

use std::{borrow::Cow, iter};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, ExprPath, Ident, ImplGenerics, Lit,
	Meta, Path, Result, Token, TypeGenerics, WhereClause,
};

use crate::{util, DeriveTrait, DeriveWhere, Error, Item, SplitGenerics, TraitImpl};
#[cfg(feature = "zeroize-on-drop")]
use crate::{Data, SimpleType};

/// Dummy-struct implement [`Trait`](crate::Trait) for [`ZeroizeOnDrop`](https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeOnDrop.html).
pub struct ZeroizeOnDrop;

impl TraitImpl for ZeroizeOnDrop {
	fn as_str(&self) -> &'static str {
		"ZeroizeOnDrop"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::ZeroizeOnDrop {
			crate_: None,
			no_drop: false,
		}
	}

	fn parse_derive_trait(
		&self,
		_span: Span,
		list: Punctuated<Meta, Token![,]>,
	) -> Result<DeriveTrait> {
		// This is already checked in `DeriveTrait::from_stream`.
		debug_assert!(!list.is_empty());

		let mut crate_ = None;
		#[cfg_attr(not(feature = "zeroize-on-drop"), allow(unused_mut))]
		let mut no_drop = false;

		for meta in list {
			match &meta {
				Meta::Path(path) => {
					#[cfg(feature = "zeroize-on-drop")]
					if path.is_ident("no_drop") {
						// Check for duplicate `no_drop` option.
						if !no_drop {
							no_drop = true;
						} else {
							return Err(Error::option_duplicate(path.span(), "no_drop"));
						}

						continue;
					}

					return Err(Error::option_trait(path.span(), self.as_str()));
				}
				Meta::NameValue(name_value) => {
					if name_value.path.is_ident("crate") {
						// Check for duplicate `crate` option.
						if crate_.is_none() {
							let path = match &name_value.value {
								Expr::Lit(ExprLit {
									lit: Lit::Str(lit_str),
									..
								}) => match lit_str.parse::<Path>() {
									Ok(path) => path,
									Err(error) => return Err(Error::path(lit_str.span(), error)),
								},
								Expr::Path(ExprPath { path, .. }) => path.clone(),
								_ => return Err(Error::option_syntax(name_value.value.span())),
							};

							if path == util::path_from_strs(&["zeroize"]) {
								return Err(Error::path_unnecessary(path.span(), "::zeroize"));
							}

							crate_ = Some(path);
						} else {
							return Err(Error::option_duplicate(name_value.span(), "crate"));
						}
					} else {
						return Err(Error::option_trait(name_value.path.span(), self.as_str()));
					}
				}
				_ => {
					return Err(Error::option_syntax(meta.span()));
				}
			}
		}

		Ok(DeriveTrait::ZeroizeOnDrop { crate_, no_drop })
	}

	#[allow(unused_variables)]
	fn additional_impl(&self, trait_: &DeriveTrait) -> Option<(Path, TokenStream)> {
		#[cfg(feature = "zeroize-on-drop")]
		return Some((trait_.path(), quote! {}));
		#[cfg(not(feature = "zeroize-on-drop"))]
		None
	}

	fn impl_item(
		&self,
		trait_: &DeriveTrait,
		imp: &ImplGenerics<'_>,
		ident: &Ident,
		ty: &TypeGenerics<'_>,
		where_clause: &Option<Cow<'_, WhereClause>>,
		body: TokenStream,
	) -> TokenStream {
		let no_drop = if let DeriveTrait::ZeroizeOnDrop { no_drop, .. } = trait_ {
			*no_drop
		} else {
			unreachable!("entered `ZeroizeOnDrop` with another trait")
		};

		let path = if no_drop {
			Path {
				leading_colon: None,
				segments: Punctuated::from_iter(iter::once(util::path_segment(
					"DeriveWhereAssertZeroizeOnDrop",
				))),
			}
		} else {
			util::path_from_strs(&["core", "ops", "Drop"])
		};

		let imp = quote! {
			impl #imp #path for #ident #ty
			#where_clause
			{
				#body
			}
		};

		if no_drop {
			quote! {
				const _: () = {
					trait DeriveWhereAssertZeroizeOnDrop {
						fn assert(&mut self);
					}

					#imp
				};
			}
		} else {
			quote! {
				#[automatically_derived]
				#imp
			}
		}
	}

	fn build_signature(
		&self,
		_derive_where: &DeriveWhere,
		item: &Item,
		_generics: &SplitGenerics<'_>,
		trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		match item {
			Item::Item(data) if data.is_empty(**trait_) => quote! {
				fn drop(&mut self) { }
			},
			#[cfg(feature = "zeroize-on-drop")]
			_ => {
				let no_drop = if let DeriveTrait::ZeroizeOnDrop { no_drop, .. } = trait_ {
					*no_drop
				} else {
					unreachable!("entered `ZeroizeOnDrop` with another trait")
				};

				if no_drop {
					let mut zeroize_on_drop = trait_.crate_();
					zeroize_on_drop
						.segments
						.push(util::path_segment("ZeroizeOnDrop"));

					quote! {
						fn assert(&mut self) {
							trait AssertZeroizeOnDrop {
								fn __derive_where_zeroize_on_drop(&mut self);
							}

							impl<T: #zeroize_on_drop + ?::core::marker::Sized> AssertZeroizeOnDrop for T {
								fn __derive_where_zeroize_on_drop(&mut self) {}
							}

							match self {
								#body
							}
						}
					}
				} else {
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
			}
			#[cfg(not(feature = "zeroize-on-drop"))]
			_ => {
				// Use unused variables.
				let _ = body;

				let path = util::path_from_root_and_strs(trait_.crate_(), &["Zeroize"]);

				quote! {
					fn drop(&mut self) {
						#path::zeroize(self);
					}
				}
			}
		}
	}

	#[cfg(feature = "zeroize-on-drop")]
	fn build_body(
		&self,
		_derive_where: &DeriveWhere,
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		match data.simple_type() {
			#[cfg(feature = "zeroize-on-drop")]
			SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
				let self_pattern = fields.self_pattern_mut();
				let self_ident = data.iter_self_ident(**trait_);

				let no_drop = if let DeriveTrait::ZeroizeOnDrop { no_drop, .. } = trait_ {
					*no_drop
				} else {
					unreachable!("entered `ZeroizeOnDrop` with another trait")
				};

				if no_drop {
					quote! {
						#self_pattern => {
							#(#self_ident.__derive_where_zeroize_on_drop();)*
						}
					}
				} else {
					quote! {
						#self_pattern => {
							#(#self_ident.zeroize_or_on_drop();)*
						}
					}
				}
			}
			#[cfg(not(feature = "zeroize-on-drop"))]
			SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
				// Use unused variables.
				let _ = fields;

				let path = util::path_from_root_and_strs(trait_.crate_(), &["Zeroize"]);

				quote! {
					#path::zeroize(self);
				}
			}
			SimpleType::Unit(_) => TokenStream::new(),
			SimpleType::Union => unreachable!("unexpected trait for union"),
		}
	}
}
