#![deny(unsafe_code)]
#![warn(clippy::cargo, clippy::missing_docs_in_private_items)]
#![cfg_attr(doc, warn(rustdoc::all), allow(rustdoc::missing_doc_code_examples))]

//! TODO

// To support a lower MSRV.
extern crate proc_macro;

use core::{cmp::Ordering, iter};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Where},
    Data, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Path, PredicateType, Result,
    Token, TraitBound, Type, TypeParamBound, WhereClause, WherePredicate,
};

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

        // Try to parse input as a WherePredicate. The problem is, both expresions
        // start with a Type, so this is the easiest way of differenciating them.
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
                    &format!("expected type to bind to, {}", error),
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
#[derive(Copy, Clone, Debug)]
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
}

impl Parse for Trait {
    fn parse(input: ParseStream) -> Result<Self> {
        use Trait::*;

        match Ident::parse(input) {
            Ok(ident) => Ok(match ident.to_string().as_str() {
                "Clone" => Clone,
                "Copy" => Copy,
                "Debug" => Debug,
                "Eq" => Eq,
                "Hash" => Hash,
                "Ord" => Ord,
                "PartialEq" => PartialEq,
                "PartialOrd" => PartialOrd,
                _ => {
                    return Err(Error::new(
                        ident.span(),
                        format!("`{}` isn't a supported trait", ident),
                    ))
                }
            }),
            Err(error) => Err(Error::new(error.span(), "expected a trait")),
        }
    }
}

impl Trait {
    /// Returns corresponding fully qualified path to the trait.
    fn path(self) -> Path {
        use Trait::*;

        syn::parse_str(match self {
            Clone => "::core::clone::Clone",
            Copy => "::core::copy::Copy",
            Debug => "::core::fmt::Debug",
            Eq => "::core::cmp::Eq",
            Hash => "::core::hash::Hash",
            Ord => "::core::cmp::Ord",
            PartialEq => "::core::cmp::PartialEq",
            PartialOrd => "::core::cmp::PartialOrd",
        })
        .expect("failed to parse path")
    }

    /// Generate `impl` item body.
    fn generate_body(self, name: &Ident, data: &Data) -> Result<TokenStream> {
        let body = match &data {
            Data::Struct(data) => {
                let pattern = name.into_token_stream();

                match &data.fields {
                    Fields::Named(fields) => {
                        self.build_for_struct(name, name, &pattern, None, fields)
                    }
                    Fields::Unnamed(fields) => {
                        self.build_for_tuple(name, name, &pattern, None, fields)
                    }
                    Fields::Unit => unreachable!("unexpected unit `struct` with generics"),
                }
            }
            Data::Enum(data) => {
                // Collect all variants to build `PartialOrd` and `Ord`.
                let variants: Vec<_> = data.variants.iter().map(|variant| &variant.ident).collect();
                let variants_type: Vec<_> = data
                    .variants
                    .iter()
                    .map(|variant| &variant.fields)
                    .collect();

                data.variants
                    .iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let debug_name = &variant.ident;
                        let pattern = quote! { #name::#debug_name };

                        match &variant.fields {
                            Fields::Named(fields) => self.build_for_struct(
                                debug_name,
                                name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                                fields,
                            ),
                            Fields::Unnamed(fields) => self.build_for_tuple(
                                debug_name,
                                name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                                fields,
                            ),
                            Fields::Unit => self.build_for_unit(
                                debug_name,
                                name,
                                &pattern,
                                Some((index, &variants, &variants_type)),
                            ),
                        }
                    })
                    .collect()
            }
            Data::Union(data) => {
                return Err(Error::new(
                    data.union_token.span(),
                    "unions aren't supported",
                ));
            }
        };

        Ok(self.build_signature(data, body))
    }

    /// Build `match` arms for [`PartialOrd`] and [`Ord`].
    fn prepare_ord(
        self,
        item_ident: &Ident,
        fields_temp: &[Ident],
        fields_other: &[Ident],
        variants: Option<(usize, &[&Ident], &[&Fields])>,
    ) -> (TokenStream, TokenStream) {
        use Trait::*;

        let path = self.path();

        let mut less = quote! { ::core::cmp::Ordering::Less };
        let mut equal = quote! { ::core::cmp::Ordering::Equal };
        let mut greater = quote! { ::core::cmp::Ordering::Greater };

        // Add `Option` to `Ordering` if we are implementing `PartialOrd`.
        match self {
            PartialOrd => {
                less = quote! { ::core::option::Option::Some(#less) };
                equal = quote! { ::core::option::Option::Some(#equal) };
                greater = quote! { ::core::option::Option::Some(#greater) };
            }
            Ord => (),
            _ => unreachable!("unsupported trait in `prepare_ord`"),
        };

        // The match arm starts with `Ordering::Equal`. This will become the
        // whole `match` arm if no fields are present.
        let mut body = quote! { #equal };

        // Builds `match` arms backwards, using the `match` arm of the field coming afterwards.
        for (field_temp, field_other) in fields_temp.iter().zip(fields_other).rev() {
            body = quote! {
                match #path::partial_cmp(#field_temp, #field_other) {
                    #equal => #body,
                    __cmp => __cmp,
                }
            };
        }

        let mut other = quote! {};

        // Build separate `match` arms to compare different variants to each
        // other. The index for these variants is used to determine which
        // `Ordering` to return.
        if let Some((variant, variants, variants_type)) = variants {
            for (index, (variants, variants_type)) in variants.iter().zip(variants_type).enumerate()
            {
                // Make sure we aren't comparing the same variant with itself.
                if variant != index {
                    let ordering = match variant.cmp(&index) {
                        Ordering::Less => &less,
                        Ordering::Equal => &equal,
                        Ordering::Greater => &greater,
                    };

                    let skip = match variants_type {
                        Fields::Named(_) => quote! { { .. } },
                        Fields::Unnamed(_) => quote! { (..) },
                        Fields::Unit => quote! {},
                    };

                    other.extend(quote! {
                        #item_ident::#variants #skip => #ordering,
                    })
                }
            }
        }

        (body, other)
    }

    /// Build method signature of the corresponding trait.
    fn build_signature(self, data: &Data, body: TokenStream) -> TokenStream {
        use Trait::*;

        match self {
            Clone => quote! {
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
            Ord => quote! {
                fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                    match self {
                        #body
                    }
                }
            },
            PartialEq => {
                let body = match data {
                    // Only check for discriminants if there is more then one variant.
                    Data::Enum(data) if data.variants.len() > 1 => {
                        // If there are no unit variants, all comparisons have to
                        // be made.
                        // `matches!` was added in 1.42.0.
                        #[allow(clippy::match_like_matches_macro)]
                        let rest = if data.variants.iter().any(|variant| {
                            if let Fields::Unit = variant.fields {
                                true
                            } else {
                                false
                            }
                        }) {
                            quote! { true }
                        } else {
                            // This follows the standard implementation.
                            quote! { unsafe { ::core::hint::unreachable_unchecked() } }
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
                };

                quote! {
                    fn eq(&self, __other: &Self) -> bool {
                        #body
                    }
                }
            }
            PartialOrd => quote! {
                fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                    match self {
                        #body
                    }
                }
            },
        }
    }

    /// Build method body if type is a structure. `pattern` is used to
    /// generalize over matching against a `struct` or an `enum`: `Self` for
    /// `struct`s and `Self::Variant` for `enum`s.
    fn build_for_struct(
        self,
        debug_name: &Ident,
        item_ident: &Ident,
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
            Ord => {
                let (body, other) =
                    self.prepare_ord(item_ident, &fields_temp, &fields_other, variants);

                quote! {
                    #pattern { #(#fields: ref #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: ref #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern { #(#fields: ref #fields_temp),* }, #pattern { #(#fields: ref #fields_other),* }) => {
                    let mut __cmp = true;
                    #(__cmp &= #path::eq(#fields_temp, #fields_other);)*
                    __cmp
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(item_ident, &fields_temp, &fields_other, variants);

                quote! {
                    #pattern { #(#fields: ref #fields_temp),* } => {
                        match __other {
                            #pattern { #(#fields: ref #fields_other),* } => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    /// Build method body if type is a tuple. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_tuple(
        self,
        debug_name: &Ident,
        item_ident: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident], &[&Fields])>,
        fields: &FieldsUnnamed,
    ) -> TokenStream {
        use Trait::*;

        let path = self.path();
        let debug_name = debug_name.to_string();

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
            Debug => quote! {
                #pattern(#(ref #fields_temp),*) => {
                    let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, #debug_name);
                    #(::core::fmt::DebugTuple::field(&mut __builder, #fields_temp);)*
                    ::core::fmt::DebugTuple::finish(&mut __builder)
                }
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
                    #pattern(#(ref #fields_temp),*) => {
                        #discriminant
                        #(#path::hash(#fields_temp, __state);)*
                    }
                }
            }
            Ord => {
                let (body, other) =
                    self.prepare_ord(item_ident, &fields_temp, &fields_other, variants);

                quote! {
                    #pattern (#(ref #fields_temp),*) => {
                        match __other {
                            #pattern (#(ref #fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {
                (#pattern(#(ref #fields_temp),*), #pattern(#(ref #fields_other),*)) => {
                    let mut __cmp = true;
                    #(__cmp &= #path::eq(#fields_temp, #fields_other);)*
                    __cmp
                }
            },
            PartialOrd => {
                let (body, other) =
                    self.prepare_ord(item_ident, &fields_temp, &fields_other, variants);

                quote! {
                    #pattern (#(ref #fields_temp),*) => {
                        match __other {
                            #pattern (#(ref #fields_other),*) => #body,
                            #other
                        }
                    }
                }
            }
        }
    }

    /// Build method body if type is a unit. See description for `pattern` in
    /// [`Self::build_for_struct`].
    fn build_for_unit(
        self,
        debug_name: &Ident,
        item_ident: &Ident,
        pattern: &TokenStream,
        variants: Option<(usize, &[&Ident], &[&Fields])>,
    ) -> TokenStream {
        use Trait::*;

        let debug_name = debug_name.to_string();

        match self {
            Clone => quote! { #pattern => #pattern, },
            Copy => quote! {},
            Debug => quote! { #pattern => ::core::fmt::Formatter::write_str(__f, #debug_name), },
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
                    ()
                } }
            }
            Ord => {
                let (body, other) = self.prepare_ord(item_ident, &[], &[], variants);

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
            PartialEq => quote! {},
            PartialOrd => {
                let (body, other) = self.prepare_ord(item_ident, &[], &[], variants);

                quote! {
                    #pattern => {
                        match __other {
                            #pattern => #body,
                            #other
                        }
                    }
                }
            }
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
        let body = trait_.generate_body(&ident, &data)?;
        let trait_ = trait_.path();

        // Where clauses on struct definitions are supported.
        let mut where_clause = where_clause.cloned();

        // Only create a where clause if required
        if let Some(generics) = &derive_where.generics {
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
                            bounds: iter::once(TypeParamBound::Trait(TraitBound {
                                paren_token: None,
                                modifier: syn::TraitBoundModifier::None,
                                lifetimes: None,
                                path: trait_.clone(),
                            }))
                            .collect(),
                        },
                    }));
            }
        }

        // Add implementation item to the output.
        output.extend(quote! {
            impl #impl_generics #trait_ for #ident #type_generics
            #where_clause
            {
                #body
            }
        })
    }

    Ok(output)
}

/// TODO
#[proc_macro_attribute]
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
    use trybuild::TestCases;

    #[test]
    fn ui() {
        // Skip UI tests when we are tesing MSRV.
        if let Ok(var) = std::env::var("DERIVE_WHERE_SKIP_UI") {
            if var == "1" {
                return;
            }
        }

        TestCases::new().compile_fail("tests/ui/*.rs");
    }

    #[test]
    fn struct_() -> Result<()> {
        test_derive(
            quote! { Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T },
            quote! { struct Test<T> { field: T } },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone
                {
                    fn clone(&self) -> Self {
                        match self {
                            Test { field: ref __field } => Test { field: ::core::clone::Clone::clone(__field) },
                        }
                    }
                }

                impl<T> ::core::copy::Copy for Test<T>
                where T: ::core::copy::Copy
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
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        match self {
                            Test { field: ref __field } => {
                                match __other {
                                    Test { field: ref __other_field } => match ::core::cmp::Ord::partial_cmp(__field, __other_field) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                }
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test { field: ref __field }, Test { field: ref __other_field }) => {
                                let mut __cmp = true;
                                __cmp &= ::core::cmp::PartialEq::eq(__field, __other_field);
                                __cmp
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match self {
                            Test { field: ref __field } => {
                                match __other {
                                    Test { field: ref __other_field } => match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                }
                            }
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
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
                    }
                }

                impl<T> ::core::copy::Copy for Test<T>
                where T: ::core::copy::Copy
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
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        match self {
                            Test(ref __0) => {
                                match __other {
                                    Test(ref __other_0) => match ::core::cmp::Ord::partial_cmp(__0, __other_0) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                }
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test(ref __0), Test(ref __other_0)) => {
                                let mut __cmp = true;
                                __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                __cmp
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::PartialOrd for Test<T>
                where T: ::core::cmp::PartialOrd
                {
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match self {
                            Test(ref __0) => {
                                match __other {
                                    Test(ref __other_0) => match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                }
                            }
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_() -> Result<()> {
        test_derive(
            quote! { Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd; T },
            quote! { enum Test<T> {
                A { field: T},
                B(T),
                C,
            } },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone
                {
                    fn clone(&self) -> Self {
                        match self {
                            Test::A { field: ref __field } => Test::A { field: ::core::clone::Clone::clone(__field) },
                            Test::B(ref __0) => Test::B(::core::clone::Clone::clone(__0)),
                            Test::C => Test::C,
                        }
                    }
                }

                impl<T> ::core::copy::Copy for Test<T>
                where T: ::core::copy::Copy
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
                            Test::B(ref __0) => {
                                let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "B");
                                ::core::fmt::DebugTuple::field(&mut __builder, __0);
                                ::core::fmt::DebugTuple::finish(&mut __builder)
                            }
                            Test::C => ::core::fmt::Formatter::write_str(__f, "C"),
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
                            Test::B(ref __0) => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                                ::core::hash::Hash::hash(__0, __state);
                            }
                            Test::C => {
                                ::core::hash::Hash::hash(&::core::mem::discriminant(self), __state);
                                ()
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::Ord for Test<T>
                where T: ::core::cmp::Ord
                {
                    fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
                        match self {
                            Test::A { field: ref __field } => {
                                match __other {
                                    Test::A { field: ref __other_field } => match ::core::cmp::Ord::partial_cmp(__field, __other_field) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                    Test::B(..) => ::core::cmp::Ordering::Less,
                                    Test::C => ::core::cmp::Ordering::Less,
                                }
                            }
                            Test::B(ref __0) => {
                                match __other {
                                    Test::B(ref __other_0) => match ::core::cmp::Ord::partial_cmp(__0, __other_0) {
                                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                        __cmp => __cmp,
                                    },
                                    Test::A { .. } => ::core::cmp::Ordering::Greater,
                                    Test::C => ::core::cmp::Ordering::Less,
                                }
                            }
                            Test::C => {
                                match __other {
                                    Test::C => ::core::cmp::Ordering::Equal,
                                    Test::A { .. } => ::core::cmp::Ordering::Greater,
                                    Test::B(..) => ::core::cmp::Ordering::Greater,
                                }
                            }
                        }
                    }
                }

                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A { field: ref __field }, Test::A { field: ref __other_field }) => {
                                    let mut __cmp = true;
                                    __cmp &= ::core::cmp::PartialEq::eq(__field, __other_field);
                                    __cmp
                                }
                                (Test::B(ref __0), Test::B(ref __other_0)) => {
                                    let mut __cmp = true;
                                    __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                    __cmp
                                }
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
                    fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                        match self {
                            Test::A { field: ref __field } => {
                                match __other {
                                    Test::A { field: ref __other_field } => match ::core::cmp::PartialOrd::partial_cmp(__field, __other_field) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                    Test::B(..) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                                    Test::C => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                                }
                            }
                            Test::B(ref __0) => {
                                match __other {
                                    Test::B(ref __other_0) => match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        __cmp => __cmp,
                                    },
                                    Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                                    Test::C => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                                }
                            }
                            Test::C => {
                                match __other {
                                    Test::C => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    Test::A { .. } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                                    Test::B(..) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                                }
                            }
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_eq_one_data() -> Result<()> {
        test_derive(
            quote! { PartialEq; T },
            quote! { enum Test<T> { A(T) } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        match (self, __other) {
                            (Test::A(ref __0), Test::A(ref __other_0)) => {
                                let mut __cmp = true;
                                __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                __cmp
                            }
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_eq_two_data() -> Result<()> {
        test_derive(
            quote! { PartialEq; T },
            quote! { enum Test<T> { A(T), B(T) } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) => {
                                    let mut __cmp = true;
                                    __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                    __cmp
                                }
                                (Test::B(ref __0), Test::B(ref __other_0)) => {
                                    let mut __cmp = true;
                                    __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                    __cmp
                                }
                                _ => unsafe { ::core::hint::unreachable_unchecked() },
                            }
                        } else {
                            false
                        }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_eq_unit() -> Result<()> {
        test_derive(
            quote! { PartialEq; T },
            quote! { enum Test<T> { A(T), B } },
            quote! {
                impl<T> ::core::cmp::PartialEq for Test<T>
                where T: ::core::cmp::PartialEq
                {
                    fn eq(&self, __other: &Self) -> bool {
                        if ::core::mem::discriminant(self) == ::core::mem::discriminant(__other) {
                            match (self, __other) {
                                (Test::A(ref __0), Test::A(ref __other_0)) => {
                                    let mut __cmp = true;
                                    __cmp &= ::core::cmp::PartialEq::eq(__0, __other_0);
                                    __cmp
                                }
                                _ => true,
                            }
                        } else {
                            false
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
                    fn clone(&self) -> Self {
                        match self {
                            Test(ref __0) => Test(::core::clone::Clone::clone(__0)),
                        }
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
