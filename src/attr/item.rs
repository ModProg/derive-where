//! Attribute parsing for items.

use core::ops::Deref;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Ident, Meta, NestedMeta, Path, PredicateType, Result, Token, TraitBound, Type,
    TypeParamBound, WhereClause, WherePredicate,
};

use crate::{
    util, Data, Error, Field, Input, Trait, TraitImpl, VariantData, DERIVE_WHERE, SKIP_INNER,
};

use super::Skip;

/// Attributes on item.
#[derive(Default)]
pub struct ItemAttr {
    /// [`Trait`](crate::Trait)s to skip all fields for.
    skip_inner: Skip,
    /// [`DeriveWhere`]s on this item.
    derive_wheres: Vec<DeriveWhere>,
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
                                if list.nested.len() == 1 && meta.path().is_ident(SKIP_INNER) {
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
    traits: Vec<DeriveTrait>,
    /// [generics](Generic) for where clause.
    generics: Option<Vec<Generic>>,
}

impl Parse for DeriveWhere {
    /// Parse the macro input, this should either be:
    /// - Comma separated traits.
    /// - Comma separated traits `;` Comma separated generics.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut traits = Vec::new();
        let mut generics = None;

        // Start parsing traits.
        while !input.is_empty() {
            traits.push(DeriveTrait::parse(input)?);

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
                    return Err(Error::derive_where_delimiter(error.span()));
                }
            }
        }

        Ok(Self { generics, traits })
    }
}

/// Holds a single generic [type](Type) or [type with bound](PredicateType).
enum Generic {
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
#[derive(Clone)]
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
            Err(error) => Err(Error::trait_syntax(error.span(), error)),
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
    pub fn where_bounds(&self, data: &Data) -> Punctuated<TypeParamBound, Token![+]> {
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

    /// Generate implementation for this [`Trait`].
    fn generate_impl(&self, generics: Option<&[Generic]>, input: &Input) -> Result<TokenStream> {
        let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
        let mut where_clause = where_clause.cloned();

        // Only create a where clause if required
        if let Some(generics) = generics {
            // We use the existing where clause or create a new one if required.
            let where_clause = where_clause.get_or_insert(WhereClause {
                where_token: <Token![where]>::default(),
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
                            colon_token: <Token![:]>::default(),
                            bounds: self.where_bounds(&input.data),
                        },
                    }));
            }
        }

        let name = input.ident;
        let path = self.path();
        let body = self.generate_body(input)?;

        let mut output = quote! {
            impl #impl_generics #path for #name #type_generics
            #where_clause
            {
                #body
            }
        };

        if let Some((path, body)) = self.additional_impl(self) {
            output.extend(quote! {
                impl #impl_generics #path for #name #type_generics
                #where_clause
                {
                    #body
                }
            })
        }

        Ok(output)
    }

    /// Generate `impl` item body.
    fn generate_body(&self, input: &Input) -> Result<TokenStream> {
        let name = input.ident;

        match &input.data {
            Data::Struct(data) => {
                let pattern = input.ident.to_token_stream();
                let body = self.build_for_struct(name, &pattern, &input.data, data);

                Ok(self.build_signature(input, body))
            }
            Data::Tuple(data) => {
                let pattern = input.ident.to_token_stream();
                let body = self.build_for_tuple(name, &pattern, &input.data, data);

                Ok(self.build_signature(input, body))
            }
            Data::Enum(data) => {
                let body: TokenStream = data
                    .iter()
                    .map(|variant| {
                        let ident = variant.ident;
                        let pattern = quote! { #name::#ident };

                        Ok(match &variant.data {
                            VariantData::Struct(fields) => {
                                self.build_for_struct(ident, &pattern, &input.data, fields)
                            }
                            VariantData::Tuple(fields) => {
                                self.build_for_tuple(ident, &pattern, &input.data, fields)
                            }
                            VariantData::Unit => self.build_for_unit(ident, &pattern, &input.data),
                        })
                    })
                    .collect::<Result<_>>()?;

                Ok(self.build_signature(input, body))
            }
            Data::Union(_) => self.build_for_union(input.span),
        }
    }

    /// Build signature for [`PartialEq`].
    fn build_partial_eq_signature(
        &self,
        attr: &Skip,
        data: &Data,
        body: TokenStream,
    ) -> TokenStream {
        // If we decided to skip everything, just return `true`.
        if attr.skip(self) {
            return quote! {
                true
            };
        }

        let unit_found = util::unit_found(data);

        match data {
            // Only check for discriminators if there is more than one variant.
            Data::Enum(variants) if variants.len() > 1 => {
                // If there are any unit variants, skip comparing them and instead return `true`. Otherwise panic as it should be unreachable.
                let rest = if unit_found {
                    quote! { true }
                } else {
                    #[cfg(not(feature = "safe"))]
                    // This follows the standard implementation.
                    quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                    #[cfg(feature = "safe")]
                    quote! { ::core::unreachable!("comparing variants yielded unexpected results") }
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
            // If only one variant was found and it's a unit variant, return `true`.
            Data::Enum(variants) if variants.len() == 1 && unit_found => {
                quote! {
                    true
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
    fn build_ord_signature(&self, name: &Ident, data: &Data, body: TokenStream) -> TokenStream {
        use DeriveTrait::*;

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

        let unit_found = util::unit_found(data);

        match data {
            // Only check for discriminators if there is more than one variant.
            Data::Enum(variants) if variants.len() > 1 => {
                let rest = if unit_found {
                    quote! { #equal }
                } else {
                    #[cfg(not(feature = "safe"))]
                    // This follows the standard implementation.
                    quote! { unsafe { ::core::hint::unreachable_unchecked() } }
                    #[cfg(feature = "safe")]
                    quote! { ::core::unreachable!("comparing variants yielded unexpected results") }
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
                    #[cfg(not(feature = "nightly"))]
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
                                    _ => ::core::unreachable!("comparing variants yielded unexpected results"),
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
            // If only one variant was found and it's a unit variant, return `Eq`.
            Data::Enum(variants) if variants.len() == 1 && unit_found => {
                quote! {
                    #equal
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
    fn build_signature(&self, input: &Input, body: TokenStream) -> TokenStream {
        todo!()
        /*use DeriveTrait::*;

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
            Default => quote! {
                fn default() -> Self {
                    #body
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
                let body = self.build_ord_signature(input.ident, &input.data, body);

                quote! {
                    #[inline]
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        #body
                    }
                }
            }
            PartialEq => {
                let body = self.build_partial_eq_signature(&input.data, body);

                quote! {
                    #[inline]
                    fn eq(&self, __other: &Self) -> bool {
                        #body
                    }
                }
            }
            PartialOrd => {
                let body = self.build_ord_signature(input.ident, &input.data, body);

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
        }*/
    }

    /// Build `match` arms for [`PartialOrd`] and [`Ord`].
    fn build_ord(&self, fields_temp: &[Ident], fields_other: &[Ident]) -> TokenStream {
        todo!()
        /*use Traits::*;

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

        body*/
    }

    /// Build method body if type is a structure. `pattern` is used to
    /// generalize over matching against a `struct` or an `enum`: `Self` for
    /// `struct`s and `Self::Variant` for `enum`s.
    fn build_for_struct(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: &Data,
        fields: &[Field],
    ) -> TokenStream {
        todo!()
        /*use Traits::*;

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
            Default => quote! {
                #pattern { #(#fields: #path::default()),* }
            },
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
        }*/
    }

    /// Build method body if type is a tuple. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_tuple(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: &Data,
        fields: &[Field],
    ) -> TokenStream {
        todo!()
        /*use Traits::*;

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
                #pattern(#(ref #fields_temp),*) => #pattern(#(#path::clone(#fields_temp)),*),
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
            Default => {
                let fields = fields_temp.iter().map(|_| quote! { #path::default() });

                quote! {
                    #pattern(#(#fields),*)
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
        }*/
    }

    /// Build method body if type is a unit. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_unit(
        &self,
        debug_name: &Ident,
        pattern: &TokenStream,
        variants: &Data,
    ) -> TokenStream {
        todo!()
        /*use Traits::*;

        match self {
            Clone => quote! { #pattern => #pattern, },
            Copy => quote! {},
            Debug => {
                let debug_name = debug_name.to_string();

                quote! { #pattern => ::core::fmt::Formatter::write_str(__f, #debug_name), }
            }
            Default => quote! { #pattern },
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
        }*/
    }

    /// Build method body if type is a union. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_union(&self, span: Span) -> Result<TokenStream> {
        todo!()
        /*use Traits::*;

        match self {
            Clone => Ok(quote! {
                #[inline]
                fn clone(&self) -> Self {
                    struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
                    let _: __AssertCopy<Self>;
                    *self
                }
            }),
            Copy => Ok(quote! {}),
            _ => Err(SynError::new(
                span,
                "traits other then `Clone` and `Copy` aren't supported by unions",
            )),
        }*/
    }
}
