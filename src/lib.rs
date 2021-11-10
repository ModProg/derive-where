#![deny(unsafe_code)]
#![cfg_attr(feature = "nightly", feature(allow_internal_unstable))]
#![warn(clippy::cargo, clippy::missing_docs_in_private_items)]
#![cfg_attr(doc, warn(rustdoc::all), allow(rustdoc::missing_doc_code_examples))]

//! TODO

// To support a lower MSRV.
extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Where},
    Data, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, Meta,
    Path, PredicateType, Result, Token, TraitBound, Type, TypeGenerics, TypeParamBound,
    WhereClause, WherePredicate,
};
#[cfg(feature = "zeroize")]
use syn::{Lit, NestedMeta};

/// Holds a single generic [type](Type) or [type with bound](PredicateType)
enum Generic {
    /// Generic type with custom [specified bounds](PredicateType)
    CoustomBound(PredicateType),
    /// Generic [type](Type) which will be bound by the implemented trait
    NoBound(Type),
}

impl Parse for Generic {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        // Try to parse input as a `WherePredicate`. The problem is, both expressions
        // start with a Type, so this is the easiest way of differentiating them.
        match WherePredicate::parse(&fork) {
            Ok(where_predicate) => {
                // Advance input as if `WherePredicate` was parsed on it.
                input.advance_to(&fork);

                match where_predicate {
                    WherePredicate::Type(path) => Ok(Generic::CoustomBound(path)),
                    WherePredicate::Lifetime(_) => Err(Error::new(
                        where_predicate.span(),
                        "bounds on lifetimes are not supported",
                    )),
                    WherePredicate::Eq(_) => Err(Error::new(
                        where_predicate.span(),
                        "equality predicates are not supported",
                    )),
                }
            }
            Err(_) => match Type::parse(input) {
                Ok(type_) => Ok(Generic::NoBound(type_)),
                Err(error) => Err(Error::new(
                    error.span(),
                    format!("expected type to bind to, {}", error),
                )),
            },
        }
    }
}

/// Holds parsed [generics](Generic) and [traits](Trait).
struct DeriveWhere {
    /// [traits](Trait) to implement.
    traits: Vec<Trait>,
    /// [generics](Generic) for where clause.
    generics: Option<Vec<Generic>>,
}

impl Parse for DeriveWhere {
    /// Parse the macro input, this should either be:
    /// - Comma separated traits
    /// - Comma separated traits `;` Comma separated generics
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut traits = Vec::new();
        let mut generics = None;

        // Start parsing traits.
        while !input.is_empty() {
            traits.push(Trait::parse(input)?);

            if !input.is_empty() {
                let fork = input.fork();

                if <Token![;]>::parse(&fork).is_ok() {
                    input.advance_to(&fork);

                    // If we found a semi-colon, start parsing generics.
                    if !input.is_empty() {
                        generics = Some(
                            Punctuated::<Generic, Token![,]>::parse_terminated(input)?
                                .into_iter()
                                .collect(),
                        );
                    }
                } else if let Err(error) = <Token![,]>::parse(input) {
                    return Err(Error::new(error.span(), "expected `;` or `,"));
                }
            }
        }

        Ok(Self { generics, traits })
    }
}

/// Trait to implement.
#[derive(Clone)]
enum Trait {
    /// [`Clone`].
    Clone,
    /// [`Copy`].
    Copy,
    /// [`Debug`](core::fmt::Debug).
    Debug,
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
        /// Zeroize path.
        crate_: Option<Path>,
        /// Zeroize drop implementation.
        drop: bool,
    },
}

impl Parse for Trait {
    fn parse(input: ParseStream) -> Result<Self> {
        /// Common error for most paths.
        fn error(span: Span) -> Result<Trait> {
            use Trait::*;

            Err(Error::new(
                span,
                format!(
                    "expected one of {}",
                    [
                        Clone.as_str(),
                        Copy.as_str(),
                        Debug.as_str(),
                        Eq.as_str(),
                        Hash.as_str(),
                        Ord.as_str(),
                        PartialEq.as_str(),
                        PartialOrd.as_str(),
                        #[cfg(feature = "zeroize")]
                        Zeroize {
                            crate_: None,
                            drop: false,
                        }
                        .as_str()
                    ]
                    .join(", ")
                ),
            ))
        }

        match Meta::parse(input) {
            Ok(meta) => match meta {
                Meta::Path(path) => {
                    if let Some(ident) = path.get_ident() {
                        Self::from_ident(ident)
                    } else {
                        error(path.span())
                    }
                }
                Meta::List(list) => {
                    if let Some(ident) = list.path.get_ident() {
                        match Trait::from_ident(ident)? {
                            #[cfg(feature = "zeroize")]
                            Self::Zeroize {
                                mut crate_,
                                mut drop,
                            } => {
                                for nested_meta in list.nested {
                                    if let NestedMeta::Meta(meta) = &nested_meta {
                                        match meta {
                                            Meta::Path(path) => {
                                                if let Some(ident) = path.get_ident() {
                                                    if ident == "drop" {
                                                        if !drop {
                                                            drop = true;
                                                            continue;
                                                        } else {
                                                            return Err(Error::new(
                                                                ident.span(),
                                                                "duplicate `drop` option",
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                            Meta::NameValue(name_value) => {
                                                if let Some(ident) = name_value.path.get_ident() {
                                                    if ident == "crate" {
                                                        if crate_.is_none() {
                                                            if let Lit::Str(lit_str) =
                                                                &name_value.lit
                                                            {
                                                                match lit_str.parse() {
                                                                    Ok(path) => {
                                                                        crate_ = Some(path);
                                                                        continue;
                                                                    }
                                                                    Err(error) => {
                                                                        return Err(Error::new(
                                                                            error.span(),
                                                                            format!(
                                                                                "expected path, {}",
                                                                                error
                                                                            ),
                                                                        ))
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            return Err(Error::new(
                                                                ident.span(),
                                                                "duplicate `crate` option",
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                            other => {
                                                return Err(Error::new(
                                                    other.span(),
                                                    "Unexpected option syntax",
                                                ))
                                            }
                                        }
                                    }

                                    return Err(Error::new(
                                        nested_meta.span(),
                                        "`Zeroize` doesn't support this option",
                                    ));
                                }

                                Ok(Self::Zeroize { crate_, drop })
                            }
                            trait_ => {
                                return Err(Error::new(
                                    list.span(),
                                    format!("`{}` doesn't support custom options", trait_.as_str()),
                                ))
                            }
                        }
                    } else {
                        error(list.path.span())
                    }
                }
                other => Err(Error::new(other.span(), "Unexpected option syntax")),
            },
            Err(err) => error(err.span()),
        }
    }
}

impl Trait {
    /// Try to create [`Trait`] from [`Ident`].
    // MSRV doesn't support [`TryFrom`](core::convert::TryFrom).
    fn from_ident(value: &Ident) -> Result<Self> {
        use Trait::*;

        Ok(match value.to_string().as_str() {
            "Clone" => Clone,
            "Copy" => Copy,
            "Debug" => Debug,
            "Eq" => Eq,
            "Hash" => Hash,
            "Ord" => Ord,
            "PartialEq" => PartialEq,
            "PartialOrd" => PartialOrd,
            #[cfg(feature = "zeroize")]
            "Zeroize" => Zeroize {
                crate_: None,
                drop: false,
            },
            _ => {
                return Err(Error::new(
                    value.span(),
                    format!("`{}` isn't a supported trait", value),
                ))
            }
        })
    }

    /// Returns fully qualified path for the trait.
    fn path(&self) -> Path {
        use Trait::*;

        syn::parse_str(match self {
            Clone => "::core::clone::Clone",
            Copy => "::core::marker::Copy",
            Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            Ord => "::core::cmp::Ord",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
            #[cfg(feature = "zeroize")]
            Zeroize { crate_, .. } => {
                if let Some(crate_) = crate_ {
                    let mut crate_ = crate_.clone();
                    crate_
                        .segments
                        .push(syn::parse_str("Zeroize").expect("failed to parse ident"));
                    return crate_;
                } else {
                    "::zeroize::Zeroize"
                }
            }
        })
        .expect("failed to parse path")
    }

    /// Returns a [str] representation of this trait for the purpose of error messages.
    fn as_str(&self) -> &'static str {
        use Trait::*;

        match self {
            Clone => "Clone",
            Copy => "Copy",
            Debug => "Debug",
            Eq => "Eq",
            Hash => "Hash",
            Ord => "Ord",
            PartialEq => "PartialEq",
            PartialOrd => "PartialOrd",
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => "Zeroize",
        }
    }

    /// Returns where-clause bounds for the trait in respect of the item type.
    fn where_bounds(&self, data: &Data) -> Punctuated<TypeParamBound, Token![+]> {
        let mut list = Punctuated::new();

        list.push(TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: syn::TraitBoundModifier::None,
            lifetimes: None,
            path: self.path(),
        }));

        // `Clone` for unions requires the `Copy` bound.
        if let (Trait::Clone, Data::Union(..)) = (self, data) {
            list.push(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: Trait::Copy.path(),
            }))
        }

        list
    }

    /// Generate an implementation for this [trait](Self).
    fn generate_impl(
        &self,
        name: &Ident,
        data: &Data,
        generics: &Option<Vec<Generic>>,
        impl_generics: &ImplGenerics,
        type_generics: &TypeGenerics,
        where_clause: &mut Option<WhereClause>,
    ) -> Result<TokenStream> {
        let body = self.generate_body(name, data)?;

        // Only create a where clause if required
        if let Some(generics) = generics {
            // We use the existing where clause or create a new one if required.
            let where_clause = where_clause.get_or_insert(WhereClause {
                where_token: Where::default(),
                predicates: Punctuated::default(),
            });

            // Insert bounds into the `where` clause.
            for generic in generics {
                where_clause
                    .predicates
                    .push(WherePredicate::Type(match generic {
                        Generic::CoustomBound(type_bound) => type_bound.clone(),
                        Generic::NoBound(path) => PredicateType {
                            lifetimes: None,
                            bounded_ty: path.clone(),
                            colon_token: Colon::default(),
                            bounds: self.where_bounds(data),
                        },
                    }));
            }
        }

        let path = self.path();
        #[allow(unused_mut)]
        let mut output = quote! {
            impl #impl_generics #path for #name #type_generics
            #where_clause
            {
                #body
            }
        };

        #[cfg(feature = "zeroize")]
        {
            if let Trait::Zeroize { drop: true, .. } = self {
                output.extend(quote! {
                    impl #impl_generics ::core::ops::Drop for #name #type_generics
                    #where_clause
                    {
                        fn drop(&mut self) {
                            #path::zeroize(self);
                        }
                    }
                })
            }
        }

        Ok(output)
    }

    /// Generate `impl` item body.
    fn generate_body(&self, name: &Ident, data: &Data) -> Result<TokenStream> {
        match data {
            Data::Struct(data) => {
                let pattern = name.into_token_stream();

                let body = match &data.fields {
                    Fields::Named(fields) => self.build_for_struct(name, &pattern, None, fields),
                    Fields::Unnamed(fields) => self.build_for_tuple(name, &pattern, None, fields),
                    Fields::Unit => unreachable!("unexpected unit `struct` with generics"),
                };

                Ok(self.build_signature(name, None, body))
            }
            Data::Enum(data) => {
                // Collect all variants to build `PartialOrd` and `Ord`.
                let variants: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();
                let variants_type: Vec<_> = data
                    .variants
                    .iter()
                    .map(|variant| &variant.fields)
                    .collect();

                let body = data
                    .variants
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let debug_name = &variant.ident;
                        let pattern = quote! { #name::#debug_name };

                        match &variant.fields {
                            Fields::Named(fields) => self.build_for_struct(
                                debug_name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                                fields,
                            ),
                            Fields::Unnamed(fields) => self.build_for_tuple(
                                debug_name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                                fields,
                            ),
                            Fields::Unit => self.build_for_unit(
                                debug_name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                            ),
                        }
                    })
                    .collect();

                Ok(self.build_signature(name, Some((&variants, &variants_type)), body))
            }
            Data::Union(data) => self.build_for_union(data),
        }
    }

    /// Build signature for [`PartialEq`].
    fn build_partial_eq_signature(
        &self,
        variants: Option<(&[&Ident], &[&Fields])>,
        body: TokenStream,
    ) -> TokenStream {
        match variants {
            // Only check for discriminators if there is more than one variant.
            Some((variants, fields)) if variants.len() > 1 => {
                // If there is any unit variant, return `true` in the `_` pattern.
                // `matches!` was added in 1.42.0.
                #[allow(clippy::match_like_matches_macro)]
                let rest = if fields.iter().any(|field| match field {
                    Fields::Named(fields) if fields.named.is_empty() => true,
                    Fields::Unnamed(fields) if fields.unnamed.is_empty() => true,
                    Fields::Unit => true,
                    _ => false,
                }) {
                    quote! { true }
                } else {
                    #[cfg(not(feature = "safe"))]
                    // This follows the standard implementation.
                    quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                    #[cfg(feature = "safe")]
                    quote! { unreachable!("comparing variants yielded unexpected results") }
                };

                quote! {
                    if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                        match (self, __other) {
                            #body
                            _ => #rest,
                        }
                    } else {
                        false
                    }
                }
            }
            _ => {
                quote! {
                    match (self, __other) {
                        #body
                    }
                }
            }
        }
    }

    /// Build signature for [`PartialOrd`] and [`Ord`].
    fn build_ord_signature(
        &self,
        name: &Ident,
        variants: Option<(&[&Ident], &[&Fields])>,
        body: TokenStream,
    ) -> TokenStream {
        use Trait::*;

        // Rust 1.36.0 doesn't support attributes on parameters.
        #[cfg(any(feature = "nightly", not(feature = "safe")))]
        let _ = name;

        /// Generate [`TokenStream`] for a pattern skipping all fields.
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        fn skip(fields: &Fields) -> TokenStream {
            match fields {
                Fields::Named(_) => quote! { { .. } },
                Fields::Unnamed(_) => quote! { (..) },
                Fields::Unit => quote! {},
            }
        }

        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let mut less = quote! { ::core::cmp::Ordering::Less };
        let mut equal = quote! { ::core::cmp::Ordering::Equal };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let mut greater = quote! { ::core::cmp::Ordering::Greater };

        // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
        match self {
            #[cfg(any(feature = "nightly", not(feature = "safe")))]
            PartialOrd => {
                equal = quote! { ::core::option::Option::Some(#equal) };
            }
            #[cfg(all(not(feature = "nightly"), feature = "safe"))]
            PartialOrd => {
                less = quote! { ::core::option::Option::Some(#less) };
                equal = quote! { ::core::option::Option::Some(#equal) };
                greater = quote! { ::core::option::Option::Some(#greater) };
            }
            Ord => (),
            _ => unreachable!("unsupported trait in `prepare_ord`"),
        };

        match variants {
            // Only check for discriminators if there is more than one variant.
            Some((variants, fields)) if variants.len() > 1 => {
                // If there is any unit variant, return `Ordering::Equal` in the `_` pattern.
                // `matches!` was added in 1.42.0.
                #[allow(clippy::match_like_matches_macro)]
                let rest = if fields.iter().any(|field| match field {
                    Fields::Named(fields) if fields.named.is_empty() => true,
                    Fields::Unnamed(fields) if fields.unnamed.is_empty() => true,
                    Fields::Unit => true,
                    _ => false,
                }) {
                    quote! { #equal }
                } else {
                    #[cfg(not(feature = "safe"))]
                    // This follows the standard implementation.
                    quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                    #[cfg(feature = "safe")]
                    quote! { unreachable!("comparing variants yielded unexpected results") }
                };

                #[cfg(any(feature = "nightly", not(feature = "safe")))]
                {
                    let path = self.path();
                    let method = match self {
                        PartialOrd => quote! { partial_cmp },
                        Ord => quote! { cmp },
                        _ => unreachable!("unsupported trait in `prepare_ord`"),
                    };

                    #[cfg(feature = "nightly")]
                    quote! {
                        let __self_disc = ::core::intrinsics::discriminant_value(&self);
                        let __other_disc = ::core::intrinsics::discriminant_value(&__other);

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                #body
                                _ => #rest,
                            }
                        } else {
                            #path::#method(&__self_disc, &__other_disc)
                        }
                    }
                    #[cfg(not(any(feature = "nightly", feature = "safe")))]
                    quote! {
                        let __self_disc = ::core::mem::discriminant(self);
                        let __other_disc = ::core::mem::discriminant(__other);

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                #body
                                _ => #rest,
                            }
                        } else {
                            #path::#method(
                                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
                            )
                        }
                    }
                }
                #[cfg(all(not(feature = "nightly"), feature = "safe"))]
                {
                    let mut different = Vec::with_capacity(variants.len());

                    // Build separate `match` arms to compare different variants to each
                    // other. The index for these variants is used to determine which
                    // `Ordering` to return.
                    for (index, (variant, field)) in variants.iter().zip(fields).enumerate() {
                        let mut arms = Vec::with_capacity(variants.len() - 1);

                        for (index_other, (variant_other, field_other)) in
                            variants.iter().zip(fields).enumerate()
                        {
                            // Make sure we aren't comparing the same variant with itself.
                            if index != index_other {
                                use core::cmp::Ordering::*;

                                let ordering = match index.cmp(&index_other) {
                                    Less => &less,
                                    Equal => &equal,
                                    Greater => &greater,
                                };

                                let skip = skip(field_other);
                                let variant_other = &variant_other;

                                arms.push(quote! {
                                    #name::#variant_other #skip => #ordering,
                                });
                            }
                        }

                        let skip = skip(field);
                        let variant = &variant;

                        different.push(quote! {
                            #name::#variant #skip => match __other {
                                #(#arms)*
                                _ => unreachable!("comparing variants yielded unexpected results"),
                            },
                        });
                    }

                    quote! {
                        let __self_disc = ::core::mem::discriminant(self);
                        let __other_disc = ::core::mem::discriminant(__other);

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                #body
                                _ => #rest,
                            }
                        } else {
                            match self {
                                #(#different)*
                            }
                        }
                    }
                }
            }
            _ => {
                quote! {
                    match (self, __other) {
                        #body
                    }
                }
            }
        }
    }

    /// Build method signature of the corresponding trait.
    fn build_signature(
        &self,
        name: &Ident,
        variants: Option<(&[&Ident], &[&Fields])>,
        body: TokenStream,
    ) -> TokenStream {
        use Trait::*;

        match self {
            Clone => quote! {
                #[inline]
                fn clone(&self) -> Self {
                    match self {
                        #body
                    }
                }
            },
            Copy => quote! {},
            Debug => quote! {
                fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        #body
                    }
                }
            },
            Eq => quote! {},
            Hash => quote! {
                fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                    match self {
                        #body
                    }
                }
            },
            Ord => {
                let body = self.build_ord_signature(name, variants, body);

                quote! {
                    #[inline]
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        #body
                    }
                }
            }
            PartialEq => {
                let body = self.build_partial_eq_signature(variants, body);

                quote! {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        #body
                    }
                }
            }
            PartialOrd => {
                let body = self.build_ord_signature(name, variants, body);

                quote! {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #body
                    }
                }
            }
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => quote! {
                fn zeroize(&mut self) {
                    match self {
                        #body
                    }
                }
            },
        }
    }

    /// Build `match` arms for [`PartialOrd`] and [`Ord`].
    fn build_ord(&self, fields_temp: &[Ident], fields_other: &[Ident]) -> TokenStream {
        use Trait::*;

        let path = self.path();
        let mut equal = quote! { ::core::cmp::Ordering::Equal };

        // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
        let method = match self {
            PartialOrd => {
                equal = quote! { ::core::option::Option::Some(#equal) };
                quote! { partial_cmp }
            }
            Ord => quote! { cmp },
            _ => unreachable!("unsupported trait in `prepare_ord`"),
        };

        // The match arm starts with `Ordering::Equal`. This will become the
        // whole `match` arm if no fields are present.
        let mut body = quote! { #equal };

        // Builds `match` arms backwards, using the `match` arm of the field coming afterwards.
        for (field_temp, field_other) in fields_temp.iter().zip(fields_other).rev() {
            body = quote! {
                match #path::#method(#field_temp, #field_other) {
                    #equal => #body,
                    __cmp => __cmp,
                }
            };
        }

        body
    }

    /// Build method body if type is a structure. `pattern` is used to
    /// generalize over matching against a `struct` or an `enum`: `Self` for
    /// `struct`s and `Self::Variant` for `enum`s.
    fn build_for_struct(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident], &[&Fields])>,
        fields: &FieldsNamed,
    ) -> TokenStream {
        use Trait::*;

        let path = self.path();

        // Extract `Ident`s from fields.
        let fields: Vec<_> = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().expect("missing field name"))
            .collect();

        // Build temporary de-structuring variable names from field `Ident`s.
        let fields_temp: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        // Build temporary de-structuring variable names for when comparing to the
        // other value, e.g. in `PartialEq`.
        let fields_other: Vec<_> = fields
            .iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern { #(#fields: ref #fields_temp),* } => #pattern { #(#fields: #path::clone(#fields_temp)),* },
            },
            Copy => quote! {},
            Debug => {
                let debug_name = debug_name.to_string();
                let debug_fields = fields.iter().map(|field| field.to_string());

                quote! {
                    #pattern { #(#fields: ref #fields_temp),* } => {
                        let mut __builder = ::core::fmt::Formatter::debug_struct(__f, #debug_name);
                        #(::core::fmt::DebugStruct::field(&mut __builder, #debug_fields, #fields_temp);)*
                        ::core::fmt::DebugStruct::finish(&mut __builder)
                    }
                }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an `enum`.
                let discriminant = if variants.is_some() {
                    Some(quote! { #path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! {
                    #pattern { #(#fields: ref #fields_temp),* } => {
                        #discriminant
                        #(#path::hash(#fields_temp, __state);)*
                    }
                }
            }
            Ord | PartialOrd => {
                if fields.is_empty() {
                    quote! {}
                } else {
                    let body = self.build_ord(&fields_temp, &fields_other);

                    quote! {
                        (#pattern { #(#fields: ref #fields_temp),* }, #pattern { #(#fields: ref #fields_other),* }) => #body,
                    }
                }
            }
            PartialEq => {
                if fields.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        (#pattern { #(#fields: ref #fields_temp),* }, #pattern { #(#fields: ref #fields_other),* }) =>
                            true #(&& #path::eq(#fields_temp, #fields_other))*,
                    }
                }
            }
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => quote! {
                #pattern { #(#fields: ref mut #fields_temp),* } => {
                    #(#path::zeroize(#fields_temp);)*
                }
            },
        }
    }

    /// Build method body if type is a tuple. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_tuple(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident], &[&Fields])>,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Trait::*;

        let path = self.path();

        // Build temporary de-structuring variable names from field indexes.
        let fields_temp: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__{}", field))
            .collect();

        // Build temporary de-structuring variable names for when comparing to the
        // other value, e.g. in `PartialEq`.
        let fields_other: Vec<_> = (0..fields.unnamed.len())
            .into_iter()
            .map(|field| format_ident!("__other_{}", field))
            .collect();

        match self {
            Clone => quote! {
                #pattern(#(ref #fields_temp),*) => #pattern (#(#path::clone(#fields_temp)),*),
            },
            Copy => quote! {},
            Debug => {
                let debug_name = debug_name.to_string();

                quote! {
                    #pattern(#(ref #fields_temp),*) => {
                        let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, #debug_name);
                        #(::core::fmt::DebugTuple::field(&mut __builder, #fields_temp);)*
                        ::core::fmt::DebugTuple::finish(&mut __builder)
                    }
                }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an `enum`.
                let discriminant = if variants.is_some() {
                    Some(quote! { #path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! {
                    #pattern(#(ref #fields_temp),*) => {
                        #discriminant
                        #(#path::hash(#fields_temp, __state);)*
                    }
                }
            }
            Ord | PartialOrd => {
                if fields.unnamed.is_empty() {
                    quote! {}
                } else {
                    let body = self.build_ord(&fields_temp, &fields_other);

                    quote! {
                        (#pattern(#(ref #fields_temp),*), #pattern(#(ref #fields_other),*)) => #body,
                    }
                }
            }
            PartialEq => {
                if fields.unnamed.is_empty() {
                    quote! {}
                } else {
                    quote! {
                        (#pattern(#(ref #fields_temp),*), #pattern(#(ref #fields_other),*)) =>
                            true #(&& #path::eq(#fields_temp, #fields_other))*,
                    }
                }
            }
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => quote! {
                #pattern(#(ref mut #fields_temp),*) => {
                    #(#path::zeroize(#fields_temp);)*
                }
            },
        }
    }

    /// Build method body if type is a unit. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_unit(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident], &[&Fields])>,
    ) -> TokenStream {
        use Trait::*;

        match self {
            Clone => quote! { #pattern => #pattern, },
            Copy => quote! {},
            Debug => {
                let debug_name = debug_name.to_string();

                quote! { #pattern => ::core::fmt::Formatter::write_str(__f, #debug_name), }
            }
            Eq => quote! {},
            Hash => {
                // Add hashing the variant if this is an `enum`.
                let discriminant = if variants.is_some() {
                    let path = self.path();
                    Some(quote! { #path::hash(&::core::mem::discriminant(self), __state); })
                } else {
                    None
                };

                quote! { #pattern => {
                    #discriminant
                } }
            }
            Ord => quote! {},
            PartialEq => quote! {},
            PartialOrd => quote! {},
            #[cfg(feature = "zeroize")]
            Zeroize { .. } => quote! {},
        }
    }

    /// Build method body if type is a union. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_union(&self, data: &DataUnion) -> Result<TokenStream> {
        use Trait::*;

        match self {
            Clone => Ok(quote! {
                #[inline]
                fn clone(&self) -> Self {
                    struct __AssertParamIsCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
                    let _: __AssertParamIsCopy<Self>;
                    *self
                }
            }),
            Copy => Ok(quote! {}),
            _ => Err(Error::new(
                data.union_token.span(),
                "traits other then `Clone` and `Copy` aren't supported by unions",
            )),
        }
    }
}

/// Internal derive function for handling errors.
fn derive_where_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let derive_where: DeriveWhere = syn::parse2(attr)?;

    // The item needs to be added, as it is consumed by the derive. Parsing
    // consumes `item` so we save any data we can't get afterwards beforehand
    // to avoid cloning.
    let mut output = quote! { #item };
    let item_span = item.span();

    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = syn::parse2(item)?;

    if generics.params.is_empty() {
        return Err(Error::new(item_span, "derive-where doesn't support items without generics, as this can already be handled by standard `#[derive()]`"));
    }

    // Build necessary generics to construct the implementation item.
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    // Every trait needs a separate implementation.
    for trait_ in derive_where.traits {
        output.extend(trait_.generate_impl(
            &ident,
            &data,
            &derive_where.generics,
            &impl_generics,
            &type_generics,
            &mut where_clause.cloned(),
        )?)
    }

    Ok(output)
}

/// TODO
#[proc_macro_attribute]
#[cfg_attr(feature = "nightly", allow_internal_unstable(core_intrinsics))]
pub fn derive_where(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: TokenStream = item.into();

    // Redirect to `derive_where_internal`, this only convert the error
    // appropriately.
    match derive_where_internal(attr.into(), item.clone()) {
        Ok(output) => output.into(),
        Err(error) => {
            // When an error happens, we still want to emit the item, as it
            // get's consumed otherwise.
            let error = error.into_compile_error();
            let output = quote! {
                #error
                #item
            };
            output.into()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn struct_() -> Result<()> {
        test_derive(
            quote! { Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T },
            quote! { struct Test<T> { field: T } },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test { field: ref __field } => Test { field: ::core::clone::Clone::clone(__field) },
                        }
                    }
                }

                impl<T> ::core::marker::Copy for Test<T>
                where T: ::core::marker::Copy
                { }

                impl<T> ::core::fmt::Debug for Test<T>
                where T: ::core::fmt::Debug
                {
                    fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Test { field: ref __field } => {
                                let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "Test");
                                ::core::fmt::DebugStruct::field(&mut __builder, "field", __field);
                                ::core::fmt::DebugStruct::finish(&mut __builder)
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::Eq for Test<T>
                where T: ::core::cmp::Eq
                { }

                impl<T> ::core::hash::Hash for Test<T>
                where T: ::core::hash::Hash
                {
                    fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                        match self {
                            Test { field: ref __field } => { ::core::hash::Hash::hash(__field, __state); }
                        }
                    }
                }

                impl<T> ::core::cmp::Ord for Test<T>
                where T: ::core::cmp::Ord
                {
                    #[inline]
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        match (self, __other) {
                            (Test { field: ref __field }, Test { field: ref __other_field }) =>
                                match ::core::cmp::Ord::cmp(__field, __other_field) {
                                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                    __cmp => __cmp,
                                },
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test { field: ref __field }, Test { field: ref __other_field }) =>
                                true && ::core::cmp::PartialEq::eq(__field, __other_field),
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match (self, __other) {
                            (Test { field: ref __field }, Test { field: ref __other_field }) =>
                                match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    __cmp => __cmp,
                                },
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn tuple() -> Result<()> {
        test_derive(
            quote! { Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }

                impl<T> ::core::marker::Copy for Test<T>
                where T: ::core::marker::Copy
                { }

                impl<T> ::core::fmt::Debug for Test<T>
                where T: ::core::fmt::Debug
                {
                    fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Test(ref __0) => {
                                let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "Test");
                                ::core::fmt::DebugTuple::field(&mut __builder, __0);
                                ::core::fmt::DebugTuple::finish(&mut __builder)
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::Eq for Test<T>
                where T: ::core::cmp::Eq
                { }

                impl<T> ::core::hash::Hash for Test<T>
                where T: ::core::hash::Hash
                {
                    fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                        match self {
                            Test(ref __0) => { ::core::hash::Hash::hash(__0, __state); }
                        }
                    }
                }

                impl<T> ::core::cmp::Ord for Test<T>
                where T: ::core::cmp::Ord
                {
                    #[inline]
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        match (self, __other) {
                            (Test(ref __0), Test(ref __other_0)) =>
                                match ::core::cmp::Ord::cmp(__0, __other_0) {
                                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                    __cmp => __cmp,
                                },
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test(ref __0), Test(ref __other_0)) =>
                                true && ::core::cmp::PartialEq::eq(__0, __other_0),
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match (self, __other) {
                            (Test(ref __0), Test(ref __other_0)) =>
                                match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    __cmp => __cmp,
                                },
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_() -> Result<()> {
        #[cfg(feature = "nightly")]
        let discriminant = quote! {
            let __self_disc = ::core::intrinsics::discriminant_value(&self);
            let __other_disc = ::core::intrinsics::discriminant_value(&__other);
        };
        #[cfg(not(feature = "nightly"))]
        let discriminant = quote! {
            let __self_disc = ::core::mem::discriminant(self);
            let __other_disc = ::core::mem::discriminant(__other);
        };
        #[cfg(feature = "nightly")]
        let ord = quote! {
            ::core::cmp::Ord::cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let ord = quote! {
            ::core::cmp::Ord::cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let ord = quote! {
            match self {
                Test::A { .. } =>
                    match __other {
                        Test::B { .. } => ::core::cmp::Ordering::Less,
                        Test::C(..) => ::core::cmp::Ordering::Less,
                        Test::D(..) => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { .. } =>
                    match __other {
                        Test::A { .. } => ::core::cmp::Ordering::Greater,
                        Test::C(..) => ::core::cmp::Ordering::Less,
                        Test::D(..) => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::C(..) =>
                    match __other {
                        Test::A { .. } => ::core::cmp::Ordering::Greater,
                        Test::B { .. } => ::core::cmp::Ordering::Greater,
                        Test::D(..) => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::D(..) =>
                    match __other {
                        Test::A { .. } => ::core::cmp::Ordering::Greater,
                        Test::B { .. } => ::core::cmp::Ordering::Greater,
                        Test::C(..) => ::core::cmp::Ordering::Greater,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::E =>
                    match __other {
                        Test::A { .. } => ::core::cmp::Ordering::Greater,
                        Test::B { .. } => ::core::cmp::Ordering::Greater,
                        Test::C(..) => ::core::cmp::Ordering::Greater,
                        Test::D(..) => ::core::cmp::Ordering::Greater,
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };
        #[cfg(feature = "nightly")]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let partial_ord = quote! {
            match self {
                Test::A { .. } =>
                    match __other {
                        Test::B { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::C(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::D(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { .. } =>
                    match __other {
                        Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::D(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::C(..) =>
                    match __other {
                        Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::D(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::D(..) =>
                    match __other {
                        Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::E =>
                    match __other {
                        Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::D(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! { Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T },
            quote! { enum Test<T> {
                A { field: T},
                B { },
                C(T),
                D(),
                E,
            } },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test::A { field: ref __field } => Test::A { field: ::core::clone::Clone::clone(__field) },
                            Test::B { } => Test::B { },
                            Test::C(ref __0) => Test::C(::core::clone::Clone::clone(__0)),
                            Test::D() => Test::D(),
                            Test::E => Test::E,
                        }
                    }
                }

                impl<T> ::core::marker::Copy for Test<T>
                where T: ::core::marker::Copy
                { }

                impl<T> ::core::fmt::Debug for Test<T>
                where T: ::core::fmt::Debug
                {
                    fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Test::A { field: ref __field } => {
                                let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "A");
                                ::core::fmt::DebugStruct::field(&mut __builder, "field", __field);
                                ::core::fmt::DebugStruct::finish(&mut __builder)
                            }
                            Test::B { } => {
                                let mut __builder = ::core::fmt::Formatter::debug_struct(__f, "B");
                                ::core::fmt::DebugStruct::finish(&mut __builder)
                            }
                            Test::C(ref __0) => {
                                let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "C");
                                ::core::fmt::DebugTuple::field(&mut __builder, __0);
                                ::core::fmt::DebugTuple::finish(&mut __builder)
                            }
                            Test::D() => {
                                let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "D");
                                ::core::fmt::DebugTuple::finish(&mut __builder)
                            }
                            Test::E => ::core::fmt::Formatter::write_str(__f, "E"),
                        }
                    }
                }

                impl<T> ::core::cmp::Eq for Test<T>
                where T: ::core::cmp::Eq
                { }

                impl<T> ::core::hash::Hash for Test<T>
                where T: ::core::hash::Hash
                {
                    fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
                        match self {
                            Test::A { field: ref __field } => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                                ::core::hash::Hash::hash(__field, __state);
                            }
                            Test::B { } => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                            }
                            Test::C(ref __0) => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                                ::core::hash::Hash::hash(__0, __state);
                            }
                            Test::D() => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                            }
                            Test::E => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::Ord for Test<T>
                where T: ::core::cmp::Ord
                {
                    #[inline]
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
                                    match ::core::cmp::Ord::cmp(__field, __other_field) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                (Test::C(ref __0), Test::C(ref __other_0)) =>
                                    match ::core::cmp::Ord::cmp(__0, __other_0) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                _ => ::core::cmp::Ordering::Equal,
                            }
                        } else {
                            #ord
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
                                    true && ::core::cmp::PartialEq::eq(__field, __other_field),
                                (Test::C(ref __0), Test::C(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A { field: ref __field }, Test::A { field: ref __other_field }) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                (Test::C(ref __0), Test::C(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                            }
                        } else {
                            #partial_ord
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn union_() -> Result<()> {
        test_derive(
            quote! { Clone, Copy; T },
            quote! { union Test<T> {
                a: core::marker::PhantomData<T>,
                b: u8,
            } },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone + ::core::marker::Copy
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        struct __AssertParamIsCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
                        let _: __AssertParamIsCopy<Self>;
                        *self
                    }
                }

                impl<T> ::core::marker::Copy for Test<T>
                where T: ::core::marker::Copy
                { }
            },
        )
    }

    #[test]
    fn enum_one_data() -> Result<()> {
        test_derive(
            quote! { PartialEq, PartialOrd; T },
            quote! { enum Test<T> { A(T) } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test::A(ref __0), Test::A(ref __other_0)) =>
                                true && ::core::cmp::PartialEq::eq(__0, __other_0),
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match (self, __other) {
                            (Test::A(ref __0), Test::A(ref __other_0)) =>
                                match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    __cmp => __cmp,
                                },
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_two_data() -> Result<()> {
        #[cfg(not(feature = "safe"))]
        let unreachable = quote! { unsafe { ::core::hint::unreachable_unchecked() } };
        #[cfg(feature = "safe")]
        let unreachable = quote! { unreachable!("comparing variants yielded unexpected results") };
        #[cfg(feature = "nightly")]
        let discriminant = quote! {
            let __self_disc = ::core::intrinsics::discriminant_value(&self);
            let __other_disc = ::core::intrinsics::discriminant_value(&__other);
        };
        #[cfg(not(feature = "nightly"))]
        let discriminant = quote! {
            let __self_disc = ::core::mem::discriminant(self);
            let __other_disc = ::core::mem::discriminant(__other);
        };
        #[cfg(feature = "nightly")]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let partial_ord = quote! {
            match self {
                Test::A(..) =>
                    match __other {
                        Test::B(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => #unreachable,
                    },
                Test::B(..) =>
                    match __other {
                        Test::A(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => #unreachable,
                    },
            }
        };

        test_derive(
            quote! { PartialEq, PartialOrd; T },
            quote! { enum Test<T> { A(T), B(T) } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                (Test::B(ref __0), Test::B(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                _ => #unreachable,
                            }
                        } else {
                            false
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                (Test::B(ref __0), Test::B(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                _ => #unreachable,
                            }
                        } else {
                            #partial_ord
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_unit() -> Result<()> {
        #[cfg(feature = "nightly")]
        let discriminant = quote! {
            let __self_disc = ::core::intrinsics::discriminant_value(&self);
            let __other_disc = ::core::intrinsics::discriminant_value(&__other);
        };
        #[cfg(not(feature = "nightly"))]
        let discriminant = quote! {
            let __self_disc = ::core::mem::discriminant(self);
            let __other_disc = ::core::mem::discriminant(__other);
        };
        #[cfg(feature = "nightly")]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let partial_ord = quote! {
            match self {
                Test::A(..) =>
                    match __other {
                        Test::B => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B =>
                    match __other {
                        Test::A(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! { PartialEq, PartialOrd; T },
            quote! { enum Test<T> { A(T), B } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                            }
                        } else {
                            #partial_ord
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_struct_unit() -> Result<()> {
        #[cfg(feature = "nightly")]
        let discriminant = quote! {
            let __self_disc = ::core::intrinsics::discriminant_value(&self);
            let __other_disc = ::core::intrinsics::discriminant_value(&__other);
        };
        #[cfg(not(feature = "nightly"))]
        let discriminant = quote! {
            let __self_disc = ::core::mem::discriminant(self);
            let __other_disc = ::core::mem::discriminant(__other);
        };
        #[cfg(feature = "nightly")]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let partial_ord = quote! {
            match self {
                Test::A(..) =>
                    match __other {
                        Test::B(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B(..) =>
                    match __other {
                        Test::A(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! { PartialEq, PartialOrd; T },
            quote! { enum Test<T> { A(T), B() } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                            }
                        } else {
                            #partial_ord
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_tuple_unit() -> Result<()> {
        #[cfg(feature = "nightly")]
        let discriminant = quote! {
            let __self_disc = ::core::intrinsics::discriminant_value(&self);
            let __other_disc = ::core::intrinsics::discriminant_value(&__other);
        };
        #[cfg(not(feature = "nightly"))]
        let discriminant = quote! {
            let __self_disc = ::core::mem::discriminant(self);
            let __other_disc = ::core::mem::discriminant(__other);
        };
        #[cfg(feature = "nightly")]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(&__self_disc, &__other_disc)
        };
        #[cfg(not(any(feature = "nightly", feature = "safe")))]
        let partial_ord = quote! {
            ::core::cmp::PartialOrd::partial_cmp(
                &unsafe { ::core::mem::transmute::<_, isize>(__self_disc) },
                &unsafe { ::core::mem::transmute::<_, isize>(__other_disc) },
            )
        };
        #[cfg(all(not(feature = "nightly"), feature = "safe"))]
        let partial_ord = quote! {
            match self {
                Test::A(..) =>
                    match __other {
                        Test::B(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { .. } =>
                    match __other {
                        Test::A(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! { PartialEq, PartialOrd; T },
            quote! { enum Test<T> { A(T), B { } } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    true && ::core::cmp::PartialEq::eq(__0, __other_0),
                                _ => true,
                            }
                        } else {
                            false
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    #[inline]
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        #discriminant

                        if __self_disc == __other_disc {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) =>
                                    match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                            }
                        } else {
                            #partial_ord
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn no_bound() -> Result<()> {
        test_derive(
            quote! { Clone },
            quote! { struct Test<T>(u8, core::marker::PhantomData<T>); },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0, ref __1) => Test(::core::clone::Clone::clone(__0), ::core::clone::Clone::clone(__1)),
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn custom_bound() -> Result<()> {
        test_derive(
            quote! { Clone; T: Copy },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: Copy
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn where_() -> Result<()> {
        test_derive(
            quote! { Clone; T },
            quote! { struct Test<T>(T) where T: core::fmt::Debug; },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where
                    T: core::fmt::Debug,
                    T: ::core::clone::Clone
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn associated_type() -> Result<()> {
        test_derive(
            quote! { Clone; <T as core::ops::Deref>::Target },
            quote! { struct Test<T>(<T as core::ops::Deref>::Target); },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where <T as core::ops::Deref>::Target: ::core::clone::Clone
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn associated_type_custom_bound() -> Result<()> {
        test_derive(
            quote! { Clone; <T as core::ops::Deref>::Target: Copy },
            quote! { struct Test<T>(<T as core::ops::Deref>::Target); },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where <T as core::ops::Deref>::Target: Copy
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }
            },
        )
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn zeroize() -> Result<()> {
        test_derive(
            quote! { Zeroize; T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> ::zeroize::Zeroize for Test<T>
                where T: ::zeroize::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                ::zeroize::Zeroize::zeroize(__0);
                            }
                        }
                    }
                }
            },
        )
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn zeroize_drop() -> Result<()> {
        test_derive(
            quote! { Zeroize(drop); T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> ::zeroize::Zeroize for Test<T>
                where T: ::zeroize::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                ::zeroize::Zeroize::zeroize(__0);
                            }
                        }
                    }
                }

                impl<T> ::core::ops::Drop for Test<T>
                where T: ::zeroize::Zeroize
                {
                    fn drop(&mut self) {
                        ::zeroize::Zeroize::zeroize(self);
                    }
                }
            },
        )
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn zeroize_crate() -> Result<()> {
        test_derive(
            quote! { Zeroize(crate = "zeroize_"); T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                zeroize_::Zeroize::zeroize(__0);
                            }
                        }
                    }
                }
            },
        )
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn zeroize_drop_crate() -> Result<()> {
        test_derive(
            quote! { Zeroize(drop, crate = "zeroize_"); T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                zeroize_::Zeroize::zeroize(__0);
                            }
                        }
                    }
                }

                impl<T> ::core::ops::Drop for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn drop(&mut self) {
                        zeroize_::Zeroize::zeroize(self);
                    }
                }
            },
        )
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn zeroize_crate_drop() -> Result<()> {
        test_derive(
            quote! { Zeroize(crate = "zeroize_", drop); T },
            quote! { struct Test<T>(T); },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                zeroize_::Zeroize::zeroize(__0);
                            }
                        }
                    }
                }

                impl<T> ::core::ops::Drop for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn drop(&mut self) {
                        zeroize_::Zeroize::zeroize(self);
                    }
                }
            },
        )
    }

    fn test_derive(attr: TokenStream, item: TokenStream, expected: TokenStream) -> Result<()> {
        let left = derive_where_internal(attr, item.clone())?.to_string();
        let right = quote! { #item #expected }.to_string();

        assert_eq!(left, right);
        Ok(())
    }
}
