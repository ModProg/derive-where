//! [`Eq`](trait@std::cmp::Eq) implementation.

use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

use crate::{util, Data, DeriveTrait, DeriveWhere, Item, SplitGenerics, Trait, TraitImpl};

/// [`TraitImpl`] for [`Eq`](trait@std::cmp::Eq).
pub struct Eq;

impl TraitImpl for Eq {
	fn as_str() -> &'static str {
		"Eq"
	}

	fn default_derive_trait() -> DeriveTrait {
		DeriveTrait::Eq
	}

	fn path(&self) -> Path {
		util::path_from_strs(&["core", "cmp", "Eq"])
	}

	fn build_signature(
		&self,
		_derive_where: &DeriveWhere,
		_item: &Item,
		_generics: &SplitGenerics<'_>,
		body: &TokenStream,
	) -> TokenStream {
		quote! {
			fn assert_receiver_is_total_eq(&self) {
				struct __AssertEq<__T: ::core::cmp::Eq + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);

				#body
			}
		}
	}

	fn build_body(&self, _derive_where: &DeriveWhere, data: &Data) -> TokenStream {
		let types = data.iter_fields(**self).map(|field| field.type_);

		quote! {
			#(let _: __AssertEq<#types>;)*
		}
	}
}

impl Deref for Eq {
	type Target = Trait;

	fn deref(&self) -> &Self::Target {
		&Trait::Eq
	}
}
