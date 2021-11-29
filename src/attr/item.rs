//! Attribute parsing for items.

use core::ops::Deref;

use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Meta, NestedMeta, Path, PredicateType, Result, Token, TraitBound, Type,
    TypeParamBound, WherePredicate,
};

use crate::{util, Error, Item, Trait, TraitImpl, DERIVE_WHERE};

use super::Skip;

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
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut self_ = ItemAttr::default();

        for attr in attrs {
            if attr.path.is_ident(DERIVE_WHERE) {
                if let Ok(meta) = attr.parse_meta() {
                    if let Meta::List(list) = meta {
                        for nested_meta in &list.nested {
                            if let NestedMeta::Meta(meta) = nested_meta {
                                if list.nested.len() == 1 && meta.path().is_ident(Skip::SKIP_INNER)
                                {
                                    self_.skip_inner.add_attribute(meta)?;
                                } else {
                                    self_.derive_wheres.push(attr.parse_args()?)
                                }
                            } else {
                                return Err(Error::option_syntax(nested_meta.span()));
                            }
                        }
                    } else {
                        return Err(Error::option_syntax(meta.span()));
                    }
                } else {
                    self_.derive_wheres.push(attr.parse_args()?)
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut traits = Vec::new();
        let mut generics = Vec::new();

        // Start parsing traits.
        while !input.is_empty() {
            traits.push(DeriveTrait::parse(input)?);

            if !input.is_empty() {
                let fork = input.fork();

                if <Token![;]>::parse(&fork).is_ok() {
                    input.advance_to(&fork);

                    // If we found a semi-colon, start parsing generics.
                    if !input.is_empty() {
                        generics = Punctuated::<Generic, Token![,]>::parse_terminated(input)?
                            .into_iter()
                            .collect();
                    }
                } else if let Err(error) = <Token![,]>::parse(input) {
                    return Err(Error::derive_where_delimiter(error.span()));
                }
            }
        }

        Ok(Self { generics, traits })
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
            Clone => util::path(&["core", "clone", "Clone"]),
            Copy => util::path(&["core", "marker", "Copy"]),
            Debug => util::path(&["core", "fmt", "Debug"]),
            Default => util::path(&["core", "default", "Default"]),
            Eq => util::path(&["core", "cmp", "Eq"]),
            Hash => util::path(&["core", "hash", "Hash"]),
            Ord => util::path(&["core", "cmp", "Ord"]),
            PartialEq => util::path(&["core", "cmp", "PartialEq"]),
            PartialOrd => util::path(&["core", "cmp", "PartialOrd"]),
            #[cfg(feature = "zeroize")]
            Zeroize { crate_, .. } => {
                if let Some(crate_) = crate_ {
                    let mut crate_ = crate_.clone();
                    crate_.segments.push(util::path_segment("Zeroize"));
                    crate_
                } else {
                    util::path(&["zeroize", "Zeroize"])
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
