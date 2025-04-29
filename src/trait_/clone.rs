//! [`Clone`](trait@std::clone::Clone) implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{TraitBound, TraitBoundModifier, TypeParamBound};

use crate::{
	data::Field, Data, DataType, DeriveTrait, DeriveWhere, Item, SimpleType, SplitGenerics, Trait,
	TraitImpl,
};

/// Dummy-struct implement [`Trait`] for [`Clone`](trait@std::clone::Clone).
pub struct Clone;

impl TraitImpl for Clone {
	fn as_str(&self) -> &'static str {
		"Clone"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::Clone
	}

	fn supports_union(&self) -> bool {
		true
	}

	fn additional_where_bounds(&self, data: &Item) -> Option<TypeParamBound> {
		// `Clone` for unions requires the `Copy` bound.
		if let Item::Item(Data {
			type_: DataType::Union(..),
			..
		}) = data
		{
			Some(TypeParamBound::Trait(TraitBound {
				paren_token: None,
				modifier: TraitBoundModifier::None,
				lifetimes: None,
				path: Trait::Copy.default_derive_trait().path(),
			}))
		} else {
			None
		}
	}

	fn build_signature(
		&self,
		derive_where: &DeriveWhere,
		item: &Item,
		_generics: &SplitGenerics<'_>,
		_trait_: &DeriveTrait,
		body: &TokenStream,
	) -> TokenStream {
		// Special implementation for items also implementing `Copy`.
		if (derive_where.generics.is_empty() || derive_where.any_custom_bound())
			&& derive_where.contains(Trait::Copy)
		{
			return quote! {
				#[inline]
				fn clone(&self) -> Self { *self }
			};
		}

		// Special implementation for unions.
		if let Item::Item(Data {
			type_: DataType::Union(..),
			..
		}) = item
		{
			quote! {
				#[inline]
				fn clone(&self) -> Self {
					struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
					let _: __AssertCopy<Self>;
					*self
				}
			}
		} else {
			quote! {
				#[inline]
				fn clone(&self) -> Self {
					match self {
						#body
					}
				}
			}
		}
	}

	fn build_body(
		&self,
		derive_where: &DeriveWhere,
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		if (derive_where.generics.is_empty() || derive_where.any_custom_bound())
			&& derive_where.contains(Trait::Copy)
		{
			return TokenStream::new();
		}

		match data.simple_type() {
			SimpleType::Struct(fields) | SimpleType::Tuple(fields) => {
				let self_pattern = &fields.self_pattern;
				let item_path = &data.path;
				let trait_path = trait_.path();
				let default_path = DeriveTrait::Default.path();

				let fields = fields.fields.iter().map(
					|field @ Field {
					     self_ident, member, ..
					 }| {
						if field.skip(Trait::Clone) || data.skip(Trait::Clone) {
							quote!(#member: #default_path::default())
						} else {
							quote!(#member: #trait_path::clone(#self_ident))
						}
					},
				);

				quote! {
					#self_pattern => #item_path { #(#fields),* },
				}
			}
			SimpleType::Unit(pattern) => {
				quote! { #pattern => #pattern, }
			}
			SimpleType::Union => TokenStream::new(),
		}
	}
}
