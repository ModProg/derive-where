//! [`Deserialize`](https://docs.rs/serde/latest/serde/derive.Deserialize.html) implementation.

use std::{borrow::Cow, ops::Deref};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, ImplGenerics, Path, TypeGenerics, WhereClause};

use crate::{util, DeriveTrait, Trait, TraitImpl, DERIVE_WHERE};

/// [`TraitImpl`] for [`Deserialize`](https://docs.rs/serde/latest/serde/derive.Deserialize.html).
#[derive(Eq, PartialEq)]
pub struct Deserialize {
	/// [`Deserialize`](https://docs.rs/serde/latest/serde/derive.Deserialize.html) path.
	pub crate_: Option<Path>,
}

impl TraitImpl for Deserialize {
	fn as_str() -> &'static str
	where
		Self: Sized,
	{
		"Deserialize"
	}

	fn default_derive_trait() -> super::DeriveTrait
	where
		Self: Sized,
	{
		DeriveTrait::Deserialize(Self { crate_: None })
	}

	fn path(&self) -> Path {
		util::path_from_root_and_strs(self.crate_(), &["Deserialize"])
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
			#[derive(#serde::Deserialize)]
			#[#derive_where::derive_where_serde]
			#full_item
		}
	}
}

impl Deserialize {
	/// Returns the path to the root crate for this trait.
	fn crate_(&self) -> Path {
		if let Some(crate_) = &self.crate_ {
			crate_.clone()
		} else {
			util::path_from_strs(&["serde"])
		}
	}
}

impl Deref for Deserialize {
	type Target = Trait;

	fn deref(&self) -> &Self::Target {
		&Trait::Deserialize
	}
}
