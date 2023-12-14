//! [`Eq`](trait@std::cmp::Eq) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DeriveTrait, DeriveWhere, Item, SplitGenerics, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for
/// [`Eq`](trait@std::cmp::Eq).
pub struct Eq;

impl TraitImpl for Eq {
	fn as_str(&self) -> &'static str {
		"Eq"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::Eq
	}

	fn build_signature(
		&self,
		_derive_where: &DeriveWhere,
		_item: &Item,
		_generics: &SplitGenerics<'_>,
		_trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		quote! {
			#[inline]
			fn assert_receiver_is_total_eq(&self) {
				struct __AssertEq<__T: ::core::cmp::Eq + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);

				#body
			}
		}
	}

	fn build_body(
		&self,
		_derive_where: &DeriveWhere,
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		let types = data.iter_fields(**trait_).map(|field| field.type_);

		quote! {
			#(let _: __AssertEq<#types>;)*
		}
	}
}
