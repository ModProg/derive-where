#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(allow_internal_unstable))]
#![allow(clippy::tabs_in_doc_comments)]
#![warn(clippy::cargo, clippy::missing_docs_in_private_items)]
#![cfg_attr(doc, warn(rustdoc::all), allow(rustdoc::missing_doc_code_examples))]

//! # Description
//!
//! Derive macro to simplify deriving standard and other traits with custom
//! generic type bounds.
//!
//! # Usage
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Clone, Debug)]
//! struct Example<T>(PhantomData<T>);
//! ```
//!
//! This will generate trait implementations for `Example` with any `T`,
//! compared with std's derives, which would only implement these traits with
//! `T: Trait` bound to the corresponding trait.
//!
//! In addition, the following convenience options are available:
//!
//! ## Generic type bounds
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Clone; T)]
//! struct Example<T, U>(T, PhantomData<U>);
//! ```
//!
//! Separating the list of trait with a semi-colon, types to bind to can be
//! specified. This will bind implementation for `Example` to `T: Trait`.
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! trait Super: Clone {}
//!
//! #[derive(DeriveWhere)]
//! #[derive_where(Clone; T: Super)]
//! struct Example<T>(PhantomData<T>);
//! ```
//!
//! This will bind implementation for `Example` to `T: Super`. But more complex
//! trait bounds are possible:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! trait Trait {
//! 	type Type;
//! }
//!
//! struct Impl;
//!
//! impl Trait for Impl {
//! 	type Type = i32;
//! }
//!
//! #[derive(DeriveWhere)]
//! #[derive_where(Clone; T::Type)]
//! struct Example<T: Trait>(T::Type);
//! ```
//!
//! This will bind implementation for `Example` to `T::Type: Super`. Any
//! combination of options listed here can be used to satisfy a specific
//! constrain.
//!
//! It is also possible to use two separate constrain specification when
//! required:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Clone; T)]
//! #[derive_where(Debug; U)]
//! struct Example<T, U>(PhantomData<T>, PhantomData<U>);
//! ```
//!
//! ## Enum default
//!
//! Deriving [`Default`] on an enum is not possible in Rust at the moment.
//! Derive-where allows this with a `default` attribute:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Default)]
//! enum Example<T> {
//! 	#[derive_where(default)]
//! 	A(PhantomData<T>),
//! }
//! ```
//!
//! ## Skipping fields
//!
//! With a `skip` or `skip_inner` attribute fields can be skipped for traits
//! that allow it, which are: [`Debug`], [`Hash`], [`Ord`], [`PartialOrd`],
//! [`PartialEq`] and [`Zeroize`].
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Debug, PartialEq; T)]
//! struct Example<T>(#[derive_where(skip)] T);
//!
//! assert_eq!(format!("{:?}", Example(42)), "Example");
//! assert_eq!(Example(42), Example(0));
//! ```
//!
//! It is also possible to skip all fields in an item or variant if desired:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Debug)]
//! #[derive_where(skip_inner)]
//! struct StructExample<T>(T);
//!
//! assert_eq!(format!("{:?}", StructExample(42)), "StructExample");
//!
//! #[derive(DeriveWhere)]
//! #[derive_where(Debug)]
//! enum EnumExample<T> {
//! 	#[derive_where(skip_inner)]
//! 	A(T),
//! }
//!
//! assert_eq!(format!("{:?}", EnumExample::A(42)), "A");
//! ```
//!
//! Selective skipping of fields for certain traits is also an option, both in
//! `skip` and `skip_inner`:
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! #[derive(DeriveWhere)]
//! #[derive_where(Debug, PartialEq)]
//! #[derive_where(skip_inner(Debug))]
//! struct Example<T>(i32, PhantomData<T>);
//!
//! assert_eq!(format!("{:?}", Example(42, PhantomData::<()>)), "Example");
//! assert_ne!(
//! 	Example(42, PhantomData::<()>),
//! 	Example(0, PhantomData::<()>)
//! );
//! ```
//!
//! ## `Zeroize` options
//!
//! [`Zeroize`] has three options:
//! - `crate`: an item-level option which specifies a path to the `zeroize`
//!   crate in case of a re-export or rename.
//! - `drop`: an item-level option which implements [`Drop`] and uses
//!   [`Zeroize`] to erase all data from memory.
//! - `fqs`: a field -level option which will use fully-qualified-syntax instead
//!   of calling the [`zeroize`] method on `self` directly. This is to avoid
//!   ambiguity between another method also called `zeroize`.
//!
//! ```
//! # use std::marker::PhantomData;
//! # use derive_where::DeriveWhere;
//! # use zeroize_::Zeroize;
//! # // Fake `Zeroize` implementation because this crate doesn't have access to
//! # // the zeroize crate because of MSRV.
//! # mod zeroize_ {
//! # 	pub trait Zeroize {
//! # 		fn zeroize(&mut self);
//! # 	}
//! # 	impl Zeroize for i32 {
//! # 		fn zeroize(&mut self) {
//! # 			*self = 0;
//! # 		}
//! # 	}
//! # }
//! #[derive(DeriveWhere)]
//! #[derive_where(Zeroize(crate = "zeroize_", drop))]
//! struct Example(#[derive_where(Zeroize(fqs))] i32);
//!
//! impl Example {
//! 	// If we didn't specify the `fqs` option, this would lead to a compile
//! 	//error because of method ambiguity.
//! 	fn zeroize(&mut self) {
//! 		self.0 = 1;
//! 	}
//! }
//!
//! let mut test = Example(42);
//!
//! // Will call the struct method.
//! test.zeroize();
//! assert_eq!(test.0, 1);
//!
//! // WIll call the `Zeroize::zeroize` method.
//! Zeroize::zeroize(&mut test);
//! assert_eq!(test.0, 0);
//! ```
//!
//! ## Supported traits
//!
//! The following traits can be derived with derive-where:
//! - [`Clone`]
//! - [`Copy`]
//! - [`Debug`]
//! - [`Default`]
//! - [`Eq`]
//! - [`Hash`]
//! - [`Ord`]
//! - [`PartialEq`]
//! - [`PartialOrd`]
//! - [`Zeroize`]: Only available with the `zeroize` crate feature.
//!
//! ## Supported items
//!
//! Structs, tuple structs, unions and enums are supported. Derive-where tries
//! it's best to discourage usage that could be covered by std's `derive`. For
//! example unit structs and enums only containing unit variants aren't
//! supported.
//!
//! Unions only support [`Clone`] and [`Copy`].
//!
//! ## `no_std` support
//!
//! `no_std` support is provided by default.
//!
//! # Crate features
//!
//! - `nightly`: Implements [`Ord`] and [`PartialOrd`] with the help of
//!   [`core::intrinsics::discriminant_value`], which is what Rust does by
//!   default too. Without this feature [`transmute`](core::mem::transmute) is
//!   used to convert [`Discriminant`](core::mem::Discriminant) to a [`i32`],
//!   which is the underlying type.
//! - `safe`: Implements [`Ord`] and [`PartialOrd`] manually. This is much
//!   slower, but might be preferred if you don't trust derive-where. It also
//!   replaces all cases of [`core::hint::unreachable_unchecked`] in [`Ord`],
//!   [`PartialEq`] and [`PartialOrd`], which is what std uses, with
//!   [`unreachable`].
//! - `zeroize`: Allows deriving [`Zeroize`].
//!
//! # MSRV
//!
//! The current MSRV is 1.34 and is being checked by the CI. A change will be
//! accompanied by a minor version bump. If MSRV is important to you, use
//! `derive-where = "~1.x"` to pin a specific minor version to your crate.
//!
//! # Alternatives
//!
//! [derivative](https://crates.io/crates/derivative)
//! ([![Crates.io](https://img.shields.io/crates/v/derivative.svg)](https://crates.io/crates/derivative))
//! is a great alternative with many options. Notably it has no `no_std`
//! support.
//!
//! # Changelog
//!
//! See the [CHANGELOG] file for details.
//!
//! # License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//! [CHANGELOG]: https://github.com/getsentry/sentry-native/blob/main/CHANGELOG
//! [LICENSE-MIT]: https://github.com/getsentry/sentry-native/blob/main/LICENSE-MIT
//! [LICENSE-APACHE]: https://github.com/getsentry/sentry-native/blob/main/LICENSE-APACHE
//! [`Debug`]: core::fmt::Debug
//! [`Default`]: core::default::Default
//! [`Hash`]: core::hash::Hash
//! [`Zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
//! [`zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html#tymethod.zeroize

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
	data::{Data, DataType, Field, SimpleType},
	error::Error,
	input::Input,
	item::Item,
	trait_::{Trait, TraitImpl},
	util::Either,
};

/// Token used for attributes.
const DERIVE_WHERE: &str = "derive_where";

/// Item-level options:
/// - `#[derive_where(Clone, ..; T, ..)]`: Specify traits to implement and
///   optionally bounds.
///   - `#[derive_where(Zeroize(crate = "path"))]`: Specify path to [`Zeroize`]
///     trait.
///   - `#[derive_where(Zeroize(drop))]`: Implement [`Drop`] with [`Zeroize`].
/// - `#[derive_where(skip_inner(Clone, ..))]`: Skip all fields in the item.
///   Optionally specify traits to constrain skipping fields. Only works for
///   structs, for enums use this on the variant-level.
///
/// Variant-level options:
/// - `#[derive_where(default)]`: Uses this variant as the default for the
///   [`Default`](core::default::Default) implementation.
/// - `#[derive_where(skip_inner(Clone, ..))]`: Skip all fields in this variant.
///   Optionally specify traits to constrain skipping fields.
///
/// Field-level options:
/// - `#[derive_where(skip(Clone, ...))]`: Skip field. Optionally specify traits
///   to constrain skipping field.
/// - `#[derive_where(Zeroize(fqs))]`: Use fully-qualified-syntax when
///   implementing [`Zeroize`].
///
/// See the [crate](crate) level description for more details.
///
/// [`Zeroize`]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
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
