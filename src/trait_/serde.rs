//! Common functionality between
//! [`Deserialize`](https://docs.rs/serde/latest/serde/derive.Deserialize.html) and
//! [`Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html).

use proc_macro2::Span;
use syn::{
	punctuated::Punctuated, spanned::Spanned, Attribute, Expr, ExprLit, Lit, Meta, Path, Token,
};

use crate::{Error, Result, Trait};

/// Parses
pub fn parse_derive_trait(
	trait_: Trait,
	attrs: &[Attribute],
	span: Span,
	list: Option<Punctuated<Meta, Token![,]>>,
) -> Result<Option<Path>> {
	if list.is_some() {
		return Err(Error::options(span, trait_.as_str()));
	}

	let mut crate_ = None;

	for attr in attrs {
		if !attr.path().is_ident("serde") {
			continue;
		}

		if let Meta::List(list) = &attr.meta {
			if let Ok(nested) =
				list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
			{
				if nested.len() != 1 {
					continue;
				}

				let meta = &nested[0];

				if !meta.path().is_ident("crate") {
					continue;
				}

				match &meta {
					Meta::NameValue(name_value) => {
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
								_ => return Err(Error::option_syntax(name_value.value.span())),
							};

							crate_ = Some(path);
						} else {
							return Err(Error::option_duplicate(name_value.span(), "crate"));
						}
					}
					_ => {
						return Err(Error::option_syntax(meta.span()));
					}
				}
			}
		}
	}

	Ok(crate_)
}
