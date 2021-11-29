//! Saves an intermediate representation of the input.

use core::iter::FromIterator;

use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Fields, Generics, Ident, Path, PathArguments, PathSegment,
    Result,
};

use crate::{Data, Default, DeriveWhere, Error, ItemAttr, Trait, VariantAttr};

/// Parsed input.
pub struct Input<'a> {
    /// `derive_where` attributes on the item.
    pub derive_wheres: Vec<DeriveWhere>,
    /// Item [`Span`].
    pub span: Span,
    /// Generics necessary to define for an `impl`.
    pub generics: &'a Generics,
    /// Fields or variants of this item.
    pub item: Item<'a>,
}

/// Fields or variants of an item.
pub enum Item<'a> {
    /// Struct.
    Struct(Data<'a>),
    /// Tuple struct.
    Tuple(Data<'a>),
    /// Enum.
    Enum {
        /// [`struct@Ident`] of this enum.
        ident: &'a Ident,
        /// Variants of this enum.
        variants: Vec<Data<'a>>,
    },
    /// Union.
    Union(Data<'a>),
}

impl<'a> Input<'a> {
    /// Create [`Input`] from `proc_macro_derive` parameter.
    pub fn parse(
        span: Span,
        DeriveInput {
            attrs,
            ident,
            generics,
            data,
            ..
        }: &'a DeriveInput,
    ) -> Result<Self> {
        // Parse `Attribute`s on item.
        let ItemAttr {
            skip_inner,
            derive_wheres,
        } = ItemAttr::from_attrs(attrs)?;

        // Extract fields and variants of this item.
        let item = match &data {
            syn::Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    let path = Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(Some(PathSegment {
                            ident: ident.clone(),
                            arguments: PathArguments::None,
                        })),
                    };

                    Data::from_named(skip_inner, Default(true), ident, path, fields)
                        .map(Item::Struct)
                }
                Fields::Unnamed(fields) => {
                    let path = Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter(Some(PathSegment {
                            ident: ident.clone(),
                            arguments: PathArguments::None,
                        })),
                    };

                    Data::from_unnamed(skip_inner, Default(true), ident, path, fields)
                        .map(Item::Tuple)
                }
                Fields::Unit => Err(Error::unit_struct(span)),
            }?,
            syn::Data::Enum(data) => {
                let mut accumulated_defaults = Default::default();

                let variants = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let path = Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter([ident, &variant.ident].iter().map(
                                |ident| PathSegment {
                                    ident: (*ident).clone(),
                                    arguments: PathArguments::None,
                                },
                            )),
                        };

                        // Parse `Attribute`s on variant.
                        let VariantAttr {
                            default,
                            skip_inner,
                        } = VariantAttr::from_attrs(
                            &variant.attrs,
                            &derive_wheres,
                            &mut accumulated_defaults,
                        )?;

                        match &variant.fields {
                            Fields::Named(fields) => {
                                Data::from_named(skip_inner, default, &variant.ident, path, fields)
                            }
                            Fields::Unnamed(fields) => Data::from_unnamed(
                                skip_inner,
                                default,
                                &variant.ident,
                                path,
                                fields,
                            ),
                            Fields::Unit => {
                                Data::from_unit(skip_inner, default, &variant.ident, path)
                            }
                        }
                    })
                    .collect::<Result<Vec<Data>>>()?;

                // Make sure a variant has the `option` attribute if `Default` is being implemented.
                if derive_wheres.iter().any(|derive_where| {
                    derive_where
                        .traits
                        .iter()
                        .any(|trait_| **trait_ == Trait::Default)
                }) {
                    let mut default_found = false;

                    for variant in &variants {
                        if variant.default.0 {
                            default_found = true;
                            break;
                        }
                    }

                    if !default_found {
                        return Err(Error::default_missing(span));
                    }
                }

                Item::Enum { ident, variants }
            }
            syn::Data::Union(data) => {
                let path = Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(Some(PathSegment {
                        ident: ident.clone(),
                        arguments: PathArguments::None,
                    })),
                };

                Item::Union(Data::from_named(
                    skip_inner,
                    Default(true),
                    ident,
                    path,
                    &data.fields,
                )?)
            }
        };

        #[cfg(feature = "zeroize")]
        {
            if !(
                // Any generics used.
                !generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
                // `Zeroize(fqs)` is used on any field.
                || item.any_fqs()
            ) {
                return Err(Error::item(span));
            }
        }

        #[cfg(not(feature = "zeroize"))]
        {
            if !(
                // Any generics used.
                !generics.params.is_empty()
                // Any field is skipped.
                || item.any_skip()
                // `Default` is used on an enum.
                || item.any_default(&derive_wheres)
            ) {
                return Err(Error::item(span));
            }
        }

        Ok(Self {
            derive_wheres,
            span,
            generics,
            item,
        })
    }
}

impl Item<'_> {
    /// Return [`struct@Ident`] of this [`Item`].
    pub fn ident(&self) -> &Ident {
        match self {
            Item::Struct(data) => data.ident,
            Item::Tuple(data) => data.ident,
            Item::Enum { ident, .. } => ident,
            Item::Union(data) => data.ident,
        }
    }

    /// Returns `true` if any field is skipped.
    fn any_skip(&self) -> bool {
        match self {
            Item::Struct(data) | Item::Tuple(data) | Item::Union(data) => {
                data.skip_inner.any_skip()
                    || data.fields.iter().any(|field| field.attr.skip.any_skip())
            }
            Item::Enum { variants, .. } => variants.iter().any(|data| {
                data.skip_inner.any_skip()
                    || data.fields.iter().any(|field| field.attr.skip.any_skip())
            }),
        }
    }

    /// Returns `true` if any field uses `default`.
    // MSRV: `matches!` was added in 1.42.0.
    #[allow(clippy::match_like_matches_macro)]
    fn any_default(&self, derive_wheres: &[DeriveWhere]) -> bool {
        (match self {
            Item::Enum { .. } => true,
            _ => false,
        }) && derive_wheres.iter().any(|derive_where| {
            derive_where
                .traits
                .iter()
                .any(|trait_| **trait_ == Trait::Default)
        })
    }

    /// Returns `true` if any field uses `Zeroize(fqs)`.
    #[cfg(feature = "zeroize")]
    fn any_fqs(&self) -> bool {
        match self {
            Item::Struct(data) | Item::Tuple(data) | Item::Union(data) => {
                data.fields.iter().any(|field| field.attr.zeroize_fqs.0)
            }
            Item::Enum { variants, .. } => variants
                .iter()
                .any(|data| data.fields.iter().any(|field| field.attr.zeroize_fqs.0)),
        }
    }
}
