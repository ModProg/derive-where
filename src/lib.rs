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
mod trait_;
mod util;

use core::iter;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, DeriveInput, PredicateType, Result, Token,
    WhereClause, WherePredicate,
};

use attr::{Default, DeriveTrait, DeriveWhere, FieldAttr, Generic, ItemAttr, Skip, VariantAttr};
use data::{Data, DataType, Field, Member, SimpleType};
use error::Error;
use input::{Input, Item};
use trait_::{Trait, TraitImpl};
use util::Either;

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

    let input = Input::parse(span, &item)?;

    Ok(input
        .derive_wheres
        .iter()
        .flat_map(|derive_where| iter::repeat(derive_where).zip(&derive_where.traits))
        .map(|(derive_where, trait_)| {
            let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
            let mut where_clause = where_clause.cloned();

            // Only create a where clause if required
            if !derive_where.generics.is_empty() {
                // We use the existing where clause or create a new one if required.
                let where_clause = where_clause.get_or_insert(WhereClause {
                    where_token: <Token![where]>::default(),
                    predicates: Punctuated::default(),
                });

                // Insert bounds into the `where` clause.
                for generic in &derive_where.generics {
                    where_clause
                        .predicates
                        .push(WherePredicate::Type(match generic {
                            Generic::CoustomBound(type_bound) => type_bound.clone(),
                            Generic::NoBound(path) => PredicateType {
                                lifetimes: None,
                                bounded_ty: path.clone(),
                                colon_token: <Token![:]>::default(),
                                bounds: trait_.where_bounds(&input.item),
                            },
                        }));
                }
            }

            let body = {
                match &input.item {
                    Item::Item(data) => {
                        let body = trait_.build_body(trait_, data);
                        trait_.build_signature(&input.item, trait_, &body)
                    }
                    Item::Enum { variants, .. } => {
                        let body: TokenStream = variants
                            .iter()
                            .map(|data| trait_.build_body(trait_, data))
                            .collect();

                        trait_.build_signature(&input.item, trait_, &body)
                    }
                }
            };

            let ident = input.item.ident();
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
        })
        .collect())
}

#[cfg(test)]
mod test {
    use quote::quote;

    use super::*;

    #[test]
    fn struct_() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
                struct Test<T> { field: T }
            },
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

                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test { field: ::core::default::Default::default() }
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
            quote! {
                #[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
                struct Test<T>(T);
            },
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

                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test(::core::default::Default::default())
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
                Test::A { field: ref __field } =>
                    match __other {
                        Test::B { } => ::core::cmp::Ordering::Less,
                        Test::C(ref __other_0) => ::core::cmp::Ordering::Less,
                        Test::D() => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { } =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
                        Test::C(ref __other_0) => ::core::cmp::Ordering::Less,
                        Test::D() => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::C(ref __0) =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
                        Test::B { } => ::core::cmp::Ordering::Greater,
                        Test::D() => ::core::cmp::Ordering::Less,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::D() =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
                        Test::B { } => ::core::cmp::Ordering::Greater,
                        Test::C(ref __other_0) => ::core::cmp::Ordering::Greater,
                        Test::E => ::core::cmp::Ordering::Less,
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::E =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::cmp::Ordering::Greater,
                        Test::B { } => ::core::cmp::Ordering::Greater,
                        Test::C(ref __other_0) => ::core::cmp::Ordering::Greater,
                        Test::D() => ::core::cmp::Ordering::Greater,
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
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
                Test::A { field: ref __field } =>
                    match __other {
                        Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { } =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::C(ref __0) =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::D() =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::E => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::E =>
                    match __other {
                        Test::A { field: ref __other_field } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::C(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        Test::D() => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! {
                #[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
                enum Test<T> {
                    A { field: T},
                    B { },
                    C(T),
                    D(),
                    #[derive_where(default)]
                    E,
                }
            },
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

                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test::E
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
            quote! {
                #[derive_where(Clone, Copy; T)]
                union Test<T> {
                    a: core::marker::PhantomData<T>,
                    b: u8,
                }
            },
            quote! {
                impl<T> ::core::clone::Clone for Test<T>
                where T: ::core::clone::Clone + ::core::marker::Copy
                {
                    #[inline]
                    fn clone(&self) -> Self {
                        struct __AssertCopy<__T: ::core::marker::Copy + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);
                        let _: __AssertCopy<Self>;
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
    fn ignore_foreign_attribute() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(Default; T)]
                enum Test<T> {
                    #[foreign(default)]
                    A { field: T },
                    #[derive_where(default)]
                    B { field: T },
                }
            },
            quote! {
                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test::B { field: ::core::default::Default::default() }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_default_struct() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(Default; T)]
                enum Test<T> {
                    #[derive_where(default)]
                    A { field: T },
                }
            },
            quote! {
                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test::A { field: ::core::default::Default::default() }
                    }
                }
            },
        )
    }

    #[test]
    fn enum_default_tuple() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(Default; T)]
                enum Test<T> {
                    #[derive_where(default)]
                    A(T),
                }
            },
            quote! {
                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test::A(::core::default::Default::default())
                    }
                }
            },
        )
    }

    #[test]
    fn enum_default_unit() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(Default; T)]
                enum Test<T> {
                    #[derive_where(default)]
                    A,
                    B(T),
                }
            },
            quote! {
                impl<T> ::core::default::Default for Test<T>
                where T: ::core::default::Default
                {
                    fn default() -> Self {
                        Test::A
                    }
                }
            },
        )
    }

    #[test]
    fn enum_one_data() -> Result<()> {
        test_derive(
            quote! {
                #[derive_where(PartialEq, PartialOrd; T)]
                enum Test<T> { A(T) }
            },
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
        let unreachable =
            quote! { ::core::unreachable!("comparing variants yielded unexpected results") };
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
                Test::A(ref __0) =>
                    match __other {
                        Test::B(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => #unreachable,
                    },
                Test::B(ref __0) =>
                    match __other {
                        Test::A(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => #unreachable,
                    },
            }
        };

        test_derive(
            quote! {
                #[derive_where(PartialEq, PartialOrd; T)]
                enum Test<T> { A(T), B(T) }
            },
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
                Test::A(ref __0) =>
                    match __other {
                        Test::B => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B =>
                    match __other {
                        Test::A(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! {
                #[derive_where(PartialEq, PartialOrd; T)]
                enum Test<T> { A(T), B }
            },
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
                Test::A(ref __0) =>
                    match __other {
                        Test::B { } => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B { } =>
                    match __other {
                        Test::A(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! {
                #[derive_where(PartialEq, PartialOrd; T)]
                enum Test<T> { A(T), B { } }
            },
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
                Test::A(ref __0) =>
                    match __other {
                        Test::B() => ::core::option::Option::Some(::core::cmp::Ordering::Less),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
                Test::B() =>
                    match __other {
                        Test::A(ref __other_0) => ::core::option::Option::Some(::core::cmp::Ordering::Greater),
                        _ => ::core::unreachable!("comparing variants yielded unexpected results"),
                    },
            }
        };

        test_derive(
            quote! {
                #[derive_where(PartialEq, PartialOrd; T)]
                enum Test<T> { A(T), B() }
            },
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
            quote! {
                #[derive_where(Clone)]
                struct Test<T>(u8, core::marker::PhantomData<T>);
            },
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
            quote! {
                #[derive_where(Clone; T: Copy)]
                struct Test<T>(T);
            },
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
            quote! {
                #[derive_where(Clone; T)]
                struct Test<T>(T) where T: core::fmt::Debug;
            },
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
            quote! {
                #[derive_where(Clone; <T as core::ops::Deref>::Target)]
                struct Test<T>(<T as core::ops::Deref>::Target);
            },
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
            quote! {
                #[derive_where(Clone; <T as core::ops::Deref>::Target: Copy)]
                struct Test<T>(<T as core::ops::Deref>::Target);
            },
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
            quote! {
                #[derive_where(Zeroize; T)]
                struct Test<T>(T);
            },
            quote! {
                impl<T> ::zeroize::Zeroize for Test<T>
                where T: ::zeroize::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                __0.zeroize();
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
            quote! {
                #[derive_where(Zeroize(drop); T)]
                struct Test<T>(T);
            },
            quote! {
                impl<T> ::zeroize::Zeroize for Test<T>
                where T: ::zeroize::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                __0.zeroize();
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
            quote! {
                #[derive_where(Zeroize(crate = "zeroize_"); T)]
                struct Test<T>(T);
            },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                __0.zeroize();
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
            quote! {
                #[derive_where(Zeroize(drop, crate = "zeroize_"); T)]
                struct Test<T>(T);
            },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                __0.zeroize();
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
            quote! {
                #[derive_where(Zeroize(crate = "zeroize_", drop); T)]
                struct Test<T>(T);
            },
            quote! {
                impl<T> zeroize_::Zeroize for Test<T>
                where T: zeroize_::Zeroize
                {
                    fn zeroize(&mut self) {
                        match self {
                            Test(ref mut __0) => {
                                __0.zeroize();
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

    fn test_derive(input: TokenStream, expected: TokenStream) -> Result<()> {
        let left = derive_where_internal(input)?.to_string();
        let right = quote! { #expected }.to_string();

        assert_eq!(left, right);
        Ok(())
    }
}
