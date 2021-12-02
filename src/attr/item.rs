//! Attribute parsing for items.

use core::ops::Deref;

use proc_macro2::Span;
use syn::{
	parse::{discouraged::Speculative, Parse, ParseStream},
	punctuated::Punctuated,
	spanned::Spanned,
	Attribute, Data, Meta, NestedMeta, Path, PredicateType, Result, Token, TraitBound, Type,
	TypeParamBound, WherePredicate,
};

use crate::{util, Error, Item, Skip, Trait, TraitImpl, DERIVE_WHERE};

/// Attributes on item.
#[derive(Default)]
pub struct ItemAttr {
	/// [`Trait`](crate::Trait)s to skip all fields for.
	pub skip_inner: Skip,
	/// [`DeriveWhere`]s on this item.
	pub derive_wheres: Vec<DeriveWhere>,
}

impl ItemAttr {
	/// Create [`ItemAttr`] from [`Attribute`]s.
	pub fn from_attrs(span: Span, data: &Data, attrs: &[Attribute]) -> Result<Self> {
		let mut self_ = ItemAttr::default();
		let mut skip_inner = None;

		for attr in attrs {
			if attr.path.is_ident(DERIVE_WHERE) {
				if let Ok(meta) = attr.parse_meta() {
					if let Meta::List(list) = meta {
						match list.nested.len() {
							// Don't allow empty list.
							0 => return Err(Error::empty(list.span())),
							// Allow `skip_inner` if list only has one item.
							1 => match list
								.nested
								.into_iter()
								.next()
								.expect("unexpected empty list")
							{
								NestedMeta::Meta(meta) => {
									if meta.path().is_ident(Skip::SKIP_INNER) {
										// Don't allow `skip_inner` on the item level for enums.
										if let Data::Enum(_) = data {
											return Err(Error::option_enum_skip_inner(meta.span()));
										} else {
											skip_inner = Some(meta);
										}
									} else {
										self_.derive_wheres.push(attr.parse_args()?);
									}
								}
								nested_meta => {
									return Err(Error::option_syntax(nested_meta.span()))
								}
							},
							_ => self_.derive_wheres.push(attr.parse_args()?),
						}
					} else {
						return Err(Error::option_syntax(meta.span()));
					}
				} else {
					self_.derive_wheres.push(attr.parse_args()?)
				}
			}
		}

		// Delay parsing of `skip_inner` to get access to all traits to be implemented.
		if let Some(meta) = skip_inner {
			self_
				.skip_inner
				.add_attribute(&self_.derive_wheres, None, &meta)?;
		}

		if self_.derive_wheres.is_empty() {
			return Err(Error::none(span));
		}

		if let Data::Union(_) = data {
			for derive_where in &self_.derive_wheres {
				for trait_ in &derive_where.traits {
					if !trait_.supports_union() {
						return Err(Error::union(span));
					}
				}
			}
		}

		Ok(self_)
	}
}

/// Holds parsed [generics](Generic) and [traits](crate::Trait).
pub struct DeriveWhere {
	/// [traits](DeriveTrait) to implement.
	pub traits: Vec<DeriveTrait>,
	/// [generics](Generic) for where clause.
	pub generics: Vec<Generic>,
}

impl Parse for DeriveWhere {
	/// Parse the macro input, this should either be:
	/// - Comma separated traits.
	/// - Comma separated traits `;` Comma separated generics.
	fn parse(input: ParseStream) -> Result<Self> {
		let mut traits = Vec::new();
		let mut generics = Vec::new();

		// Start parsing traits.
		while !input.is_empty() {
			traits.push(DeriveTrait::parse(input)?);

			if !input.is_empty() {
				let mut fork = input.fork();
				let mut delimiter_found = None;

				match <Token![,]>::parse(&fork) {
					Ok(_) => {
						input.advance_to(&fork);
					}
					Err(error) => {
						delimiter_found = Some(error.span());
						fork = input.fork();
					}
				}

				if <Token![;]>::parse(&fork).is_ok() {
					input.advance_to(&fork);

					// If we found a semi-colon, start parsing generics.
					if !input.is_empty() {
						generics = Punctuated::<Generic, Token![,]>::parse_terminated(input)?
							.into_iter()
							.collect();
					}
				} else if let Some(span) = delimiter_found {
					return Err(Error::derive_where_delimiter(span));
				}
			}
		}

		Ok(Self { generics, traits })
	}
}

impl DeriveWhere {
	/// Returns selected [`DeriveTrait`] if present.
	pub fn trait_(&self, trait_: Trait) -> Option<&DeriveTrait> {
		self.traits
			.iter()
			.find(|derive_trait| ***derive_trait == trait_)
	}
}

/// Holds a single generic [type](Type) or [type with bound](PredicateType).
pub enum Generic {
	/// Generic type with custom [specified bounds](PredicateType).
	CoustomBound(PredicateType),
	/// Generic [type](Type) which will be bound to the [`DeriveTrait`].
	NoBound(Type),
}

impl Parse for Generic {
	fn parse(input: ParseStream) -> Result<Self> {
		let fork = input.fork();

		// Try to parse input as a `WherePredicate`. The problem is, both expressions
		// start with a Type, so this is the easiest way of differentiating them.
		if let Ok(where_predicate) = WherePredicate::parse(&fork) {
			// Advance input as if `WherePredicate` was parsed on it.
			input.advance_to(&fork);

			if let WherePredicate::Type(path) = where_predicate {
				Ok(Generic::CoustomBound(path))
			} else {
				Err(Error::generic(where_predicate.span()))
			}
		} else {
			match Type::parse(input) {
				Ok(type_) => Ok(Generic::NoBound(type_)),
				Err(error) => Err(Error::generic_syntax(error.span(), error)),
			}
		}
	}
}

/// Trait to implement.
pub enum DeriveTrait {
	/// [`Clone`].
	Clone,
	/// [`Copy`].
	Copy,
	/// [`Debug`](core::fmt::Debug).
	Debug,
	/// [`Default`].
	Default,
	/// [`Eq`].
	Eq,
	/// [`Hash`](core::hash::Hash).
	Hash,
	/// [`Ord`].
	Ord,
	/// [`PartialEq`].
	PartialEq,
	/// [`PartialOrd`].
	PartialOrd,
	/// [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html).
	#[cfg(feature = "zeroize")]
	Zeroize {
		/// [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html) path.
		crate_: Option<Path>,
		/// [`Zeroize`](https://docs.rs/zeroize/1.4.3/zeroize/trait.Zeroize.html) [`Drop`] implementation.
		drop: bool,
	},
}

impl Parse for DeriveTrait {
	fn parse(input: ParseStream) -> Result<Self> {
		match Meta::parse(input) {
			Ok(meta) => {
				let trait_ = Trait::from_path(meta.path())?;

				match meta {
					Meta::Path(_) => Ok(trait_.default_derive_trait()),
					Meta::List(list) => trait_.parse_derive_trait(list),
					Meta::NameValue(name_value) => Err(Error::option_syntax(name_value.span())),
				}
			}
			Err(error) => Err(Error::trait_syntax(error.span())),
		}
	}
}

impl Deref for DeriveTrait {
	type Target = Trait;

	fn deref(&self) -> &Self::Target {
		use DeriveTrait::*;

		match self {
			Clone => &Trait::Clone,
			Copy => &Trait::Copy,
			Debug => &Trait::Debug,
			Default => &Trait::Default,
			Eq => &Trait::Eq,
			Hash => &Trait::Hash,
			Ord => &Trait::Ord,
			PartialEq => &Trait::PartialEq,
			PartialOrd => &Trait::PartialOrd,
			#[cfg(feature = "zeroize")]
			Zeroize { .. } => &Trait::Zeroize,
		}
	}
}

impl DeriveTrait {
	/// Returns fully qualified path for this trait.
	pub fn path(&self) -> Path {
		use DeriveTrait::*;

		match self {
			Clone => util::path_from_strs(&["core", "clone", "Clone"]),
			Copy => util::path_from_strs(&["core", "marker", "Copy"]),
			Debug => util::path_from_strs(&["core", "fmt", "Debug"]),
			Default => util::path_from_strs(&["core", "default", "Default"]),
			Eq => util::path_from_strs(&["core", "cmp", "Eq"]),
			Hash => util::path_from_strs(&["core", "hash", "Hash"]),
			Ord => util::path_from_strs(&["core", "cmp", "Ord"]),
			PartialEq => util::path_from_strs(&["core", "cmp", "PartialEq"]),
			PartialOrd => util::path_from_strs(&["core", "cmp", "PartialOrd"]),
			#[cfg(feature = "zeroize")]
			Zeroize { crate_, .. } => {
				if let Some(crate_) = crate_ {
					let mut crate_ = crate_.clone();
					crate_.segments.push(util::path_segment("Zeroize"));
					crate_
				} else {
					util::path_from_strs(&["zeroize", "Zeroize"])
				}
			}
		}
	}

	/// Returns where-clause bounds for the trait in respect of the item type.
	pub fn where_bounds(&self, data: &Item) -> Punctuated<TypeParamBound, Token![+]> {
		let mut list = Punctuated::new();

		list.push(TypeParamBound::Trait(TraitBound {
			paren_token: None,
			modifier: syn::TraitBoundModifier::None,
			lifetimes: None,
			path: self.path(),
		}));

		if let Some(bound) = self.additional_where_bounds(data) {
			list.push(bound)
		}

		list
	}
}
