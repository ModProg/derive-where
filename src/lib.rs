#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(allow_internal_unstable))]
#![warn(clippy::cargo, clippy::missing_docs_in_private_items)]
#![cfg_attr(doc, warn(rustdoc::all), allow(rustdoc::missing_doc_code_examples))]

//! TODO

// MSRV: needed to support a lower MSRV.
extern crate proc_macro;

mod attr;
mod data;
mod error;
mod input;
mod item;
#[cfg(test)]
mod test;
mod trait_;
mod util;

use std::{borrow::Cow, iter};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Generics, Result};

#[cfg(feature = "zeroize")]
use self::attr::ZeroizeFqs;
use self::{
	attr::{Default, DeriveTrait, DeriveWhere, FieldAttr, ItemAttr, Skip, VariantAttr},
	data::{Data, DataType, Field, Member, SimpleType},
	error::Error,
	input::Input,
	item::Item,
	trait_::{Trait, TraitImpl},
	util::Either,
};

/// Token used for attributes.
const DERIVE_WHERE: &str = "derive_where";

/// TODO
#[proc_macro_derive(DeriveWhere, attributes(derive_where))]
#[cfg_attr(feature = "nightly", allow_internal_unstable(core_intrinsics))]
pub fn derive_where(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	match derive_where_internal(input.into()) {
		Ok(output) => output.into(),
		Err(error) => error.into_compile_error().into(),
	}
}

/// Internal derive function for handling errors.
fn derive_where_internal(input: TokenStream) -> Result<TokenStream> {
	// Save `Span` before we consume `input` when parsing it.
	let span = input.span();
	let item = syn::parse2::<DeriveInput>(input).expect("derive on unparsable item");

	let Input {
		derive_wheres,
		generics,
		item,
	} = Input::from_input(span, &item)?;

	Ok(derive_wheres
		.iter()
		.flat_map(|derive_where| iter::repeat(derive_where).zip(&derive_where.traits))
		.map(|(derive_where, trait_)| generate_impl(derive_where, trait_, &item, generics))
		.collect())
}

/// Generate implementation for a [`Trait`].
fn generate_impl(
	derive_where: &DeriveWhere,
	trait_: &DeriveTrait,
	item: &Item,
	generics: &Generics,
) -> TokenStream {
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
	let mut where_clause = where_clause.map(Cow::Borrowed);
	derive_where.where_clause(&mut where_clause, trait_, item);

	let body = generate_body(trait_, item);

	let ident = item.ident();
	let path = trait_.path();
	let mut output = quote! {
		impl #impl_generics #path for #ident #type_generics
		#where_clause
		{
			#body
		}
	};

	if let Some((path, body)) = trait_.additional_impl(trait_) {
		output.extend(quote! {
			impl #impl_generics #path for #ident #type_generics
			#where_clause
			{
				#body
			}
		})
	}

	output
}

/// Generate implementation method body for a [`Trait`].
fn generate_body(trait_: &DeriveTrait, item: &Item) -> TokenStream {
	match &item {
		Item::Item(data) => {
			let body = trait_.build_body(trait_, data);
			trait_.build_signature(item, trait_, &body)
		}
		Item::Enum { variants, .. } => {
			let body: TokenStream = variants
				.iter()
				.map(|data| trait_.build_body(trait_, data))
				.collect();

			trait_.build_signature(item, trait_, &body)
		}
	}
}
