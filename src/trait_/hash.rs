//! [`Hash`](trait@std::hash::Hash) implementation.

use proc_macro2::TokenStream;
use quote::quote;

use crate::{Data, DataType, DeriveTrait, DeriveWhere, Item, SimpleType, SplitGenerics, TraitImpl};

/// Dummy-struct implement [`Trait`](crate::Trait) for
/// [`Hash`](trait@std::hash::Hash).
pub struct Hash;

impl TraitImpl for Hash {
	fn as_str(&self) -> &'static str {
		"Hash"
	}

	fn default_derive_trait(&self) -> DeriveTrait {
		DeriveTrait::Hash
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
			fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
				match self {
					#body
				}
			}
		}
	}

	fn build_body(
		&self,
		_derive_where: &DeriveWhere,
		trait_: &DeriveTrait,
		data: &Data,
	) -> TokenStream {
		let self_pattern = data.self_pattern();
		let trait_path = trait_.path();

		// Add hashing the variant if this is an enum.
		let discriminant = if let DataType::Variant { .. } = data.type_ {
			Some(quote! { #trait_path::hash(&::core::mem::discriminant(self), __state); })
		} else {
			None
		};

		match data.simple_type() {
			SimpleType::Struct(_) | SimpleType::Tuple(_) => {
				let self_ident = data.iter_self_ident(**trait_);

				quote! {
					#self_pattern => {
						#discriminant
						#(#trait_path::hash(#self_ident, __state);)*
					}
				}
			}
			SimpleType::Unit(_) => {
				quote! {
					#self_pattern => {
						#discriminant
					}
				}
			}
			SimpleType::Union => unreachable!("unexpected trait for union"),
		}
	}
}
