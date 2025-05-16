//! Common functionality between
//! [`Deserialize`](https://docs.rs/serde/latest/serde/derive.Deserialize.html) and
//! [`Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html).

use proc_macro2::Span;
use syn::{
	punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, ExprPath, Lit, Meta, Path, Token,
};

use crate::{util, Error, Result, Trait};

/// Parses
pub fn parse_derive_trait(
	trait_: Trait,
	_span: Span,
	list: Punctuated<Meta, Token![,]>,
) -> Result<Option<Path>> {
	// This is already checked in `DeriveTrait::from_stream`.
	debug_assert!(!list.is_empty());

	let mut crate_ = None;

	for meta in list {
		match &meta {
			Meta::Path(path) => {
				return Err(Error::option_trait(path.span(), trait_.as_str()));
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

						if path == util::path_from_strs(&["serde"]) {
							return Err(Error::path_unnecessary(path.span(), "::serde"));
						}

						crate_ = Some(path);
					} else {
						return Err(Error::option_duplicate(name_value.span(), "crate"));
					}
				} else {
					return Err(Error::option_trait(name_value.path.span(), trait_.as_str()));
				}
			}
			_ => {
				return Err(Error::option_syntax(meta.span()));
			}
		}
	}

	Ok(crate_)
}
