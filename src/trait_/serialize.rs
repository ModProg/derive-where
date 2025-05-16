//! [`Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html) implementation.

use std::{borrow::Cow, ops::Deref};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	punctuated::Punctuated, DeriveInput, Ident, ImplGenerics, Meta, Path, Result, TypeGenerics,
	WhereClause,
};

use super::serde;
use crate::{util, DeriveTrait, Trait, TraitImpl, DERIVE_WHERE};

/// [`TraitImpl`] for [`Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html).
#[derive(Eq, PartialEq)]
pub struct Serialize {
	/// [`Serialize`](https://docs.rs/serde/latest/serde/derive.Serialize.html) path.
	pub crate_: Option<Path>,
}

impl TraitImpl for Serialize {
	fn as_str() -> &'static str
	where
		Self: Sized,
	{
		"Serialize"
	}

	fn default_derive_trait() -> super::DeriveTrait
	where
		Self: Sized,
	{
		DeriveTrait::Serialize(Self { crate_: None })
	}

	fn parse_derive_trait(span: Span, list: Punctuated<Meta, syn::Token![,]>) -> Result<DeriveTrait>
	where
		Self: Sized,
	{
		Ok(DeriveTrait::Serialize(Self {
			crate_: serde::parse_derive_trait(Trait::Serialize, span, list)?,
		}))
	}

	fn path(&self) -> Path {
		util::path_from_root_and_strs(self.crate_(), &["Serialize"])
	}

	fn impl_item(
		&self,
		crate_: Option<&Path>,
		full_item: &DeriveInput,
		_: &ImplGenerics<'_>,
		_: &Ident,
		_: &TypeGenerics<'_>,
		_: &Option<Cow<'_, WhereClause>>,
		_: TokenStream,
	) -> TokenStream {
		let derive_where = crate_
			.map(Cow::Borrowed)
			.unwrap_or_else(|| Cow::Owned(util::path_from_strs(&[DERIVE_WHERE])));
		let serde = self.crate_();

		quote! {
			#[derive(#serde::Serialize)]
			#[#derive_where::derive_where_serde]
			#full_item
		}
	}
}

impl Serialize {
	/// Returns the path to the root crate for this trait.
	fn crate_(&self) -> Path {
		if let Some(crate_) = &self.crate_ {
			crate_.clone()
		} else {
			util::path_from_strs(&["serde"])
		}
	}
}

impl Deref for Serialize {
	type Target = Trait;

	fn deref(&self) -> &Self::Target {
		&Trait::Serialize
	}
}
