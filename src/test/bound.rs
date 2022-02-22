use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; T)]
			struct Test<T, U>(T, std::marker::PhantomData<U>);
		},
		quote! {
			impl<T, U> ::core::clone::Clone for Test<T, U>
			where T: ::core::clone::Clone
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
fn bound_multiple() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; T, U)]
			struct Test<T, U, V>((T, U), std::marker::PhantomData<V>);
		},
		quote! {
			impl<T, U, V> ::core::clone::Clone for Test<T, U, V>
			where
				T: ::core::clone::Clone,
				U: ::core::clone::Clone
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
			struct Test<T, U>(T, std::marker::PhantomData<U>) where T: std::fmt::Debug;
		},
		quote! {
			impl<T, U> ::core::clone::Clone for Test<T, U>
			where
				T: std::fmt::Debug,
				T: ::core::clone::Clone
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
fn associated_type() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; <T as std::ops::Deref>::Target)]
			struct Test<T>(<T as std::ops::Deref>::Target);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where <T as std::ops::Deref>::Target: ::core::clone::Clone
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
			#[derive_where(Clone; <T as std::ops::Deref>::Target: Copy)]
			struct Test<T>(<T as std::ops::Deref>::Target);
		},
		quote! {
			impl<T> ::core::clone::Clone for Test<T>
			where <T as std::ops::Deref>::Target: Copy
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
fn check_trait_bounds() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd; T)]
			struct Test<T, U>(T, std::marker::PhatomData<U>);
		},
		quote! {
			impl<T, U> ::core::clone::Clone for Test<T, U>
			where T: ::core::clone::Clone
			{
				#[inline]
				fn clone(&self) -> Self {
					*self
				}
			}

			impl<T, U> ::core::marker::Copy for Test<T, U>
			where T: ::core::marker::Copy
			{ }

			impl<T, U> ::core::fmt::Debug for Test<T, U>
			where T: ::core::fmt::Debug
			{
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test(ref __0, ref __1) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "Test");
							::core::fmt::DebugTuple::field(&mut __builder, __0);
							::core::fmt::DebugTuple::field(&mut __builder, __1);
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
					}
				}
			}

			impl<T, U> ::core::default::Default for Test<T, U>
			where T: ::core::default::Default
			{
				fn default() -> Self {
					Test(::core::default::Default::default(), ::core::default::Default::default())
				}
			}

			impl<T, U> ::core::cmp::Eq for Test<T, U>
			where T: ::core::cmp::Eq
			{
				#[inline]
				fn assert_receiver_is_total_eq(&self) {
					struct __AssertEq<__T: ::core::cmp::Eq + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);

					// For some reason the comparison fails without the extra space at the end.
					let _: __AssertEq<T >;
					let _: __AssertEq<std::marker::PhatomData<U> >;
				}
			}

			impl<T, U> ::core::hash::Hash for Test<T, U>
			where T: ::core::hash::Hash
			{
				fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
					match self {
						Test(ref __0, ref __1) => {
							::core::hash::Hash::hash(__0, __state);
							::core::hash::Hash::hash(__1, __state);
						}
					}
				}
			}

			impl<T, U> ::core::cmp::Ord for Test<T, U>
			where T: ::core::cmp::Ord
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::Ord::cmp(__0, __other_0) {
								::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(__1, __other_1) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}

			impl<T, U> ::core::cmp::PartialEq for Test<T, U>
			where T: ::core::cmp::PartialEq
			{
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							true
							&& ::core::cmp::PartialEq::eq(__0, __other_0)
							&& ::core::cmp::PartialEq::eq(__1, __other_1),
					}
				}
			}

			impl<T, U> ::core::cmp::PartialOrd for Test<T, U>
			where T: ::core::cmp::PartialOrd
			{
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(__1, __other_1) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}

#[test]
fn check_multiple_trait_bounds() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd; T, U)]
			struct Test<T, U, V>(T, std::marker::PhatomData<(U, V)>);
		},
		quote! {
			impl<T, U, V> ::core::clone::Clone for Test<T, U, V>
			where
				T: ::core::clone::Clone,
				U: ::core::clone::Clone
			{
				#[inline]
				fn clone(&self) -> Self {
					*self
				}
			}

			impl<T, U, V> ::core::marker::Copy for Test<T, U, V>
			where
				T: ::core::marker::Copy,
				U: ::core::marker::Copy
			{ }

			impl<T, U, V> ::core::fmt::Debug for Test<T, U, V>
			where
				T: ::core::fmt::Debug,
				U: ::core::fmt::Debug
			{
				fn fmt(&self, __f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
					match self {
						Test(ref __0, ref __1) => {
							let mut __builder = ::core::fmt::Formatter::debug_tuple(__f, "Test");
							::core::fmt::DebugTuple::field(&mut __builder, __0);
							::core::fmt::DebugTuple::field(&mut __builder, __1);
							::core::fmt::DebugTuple::finish(&mut __builder)
						}
					}
				}
			}

			impl<T, U, V> ::core::default::Default for Test<T, U, V>
			where
				T: ::core::default::Default,
				U: ::core::default::Default
			{
				fn default() -> Self {
					Test(::core::default::Default::default(), ::core::default::Default::default())
				}
			}

			impl<T, U, V> ::core::cmp::Eq for Test<T, U, V>
			where
				T: ::core::cmp::Eq,
				U: ::core::cmp::Eq
			{
				#[inline]
				fn assert_receiver_is_total_eq(&self) {
					struct __AssertEq<__T: ::core::cmp::Eq + ?::core::marker::Sized>(::core::marker::PhantomData<__T>);

					// For some reason the comparison fails without the extra space at the end.
					let _: __AssertEq<T >;
					let _: __AssertEq<std::marker::PhatomData<(U, V)> >;
				}
			}

			impl<T, U, V> ::core::hash::Hash for Test<T, U, V>
			where
				T: ::core::hash::Hash,
				U: ::core::hash::Hash
			{
				fn hash<__H: ::core::hash::Hasher>(&self, __state: &mut __H) {
					match self {
						Test(ref __0, ref __1) => {
							::core::hash::Hash::hash(__0, __state);
							::core::hash::Hash::hash(__1, __state);
						}
					}
				}
			}

			impl<T, U, V> ::core::cmp::Ord for Test<T, U, V>
			where
				T: ::core::cmp::Ord,
				U: ::core::cmp::Ord
			{
				#[inline]
				fn cmp(&self, __other: &Self) -> ::core::cmp::Ordering {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::Ord::cmp(__0, __other_0) {
								::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(__1, __other_1) {
									::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}

			impl<T, U, V> ::core::cmp::PartialEq for Test<T, U, V>
			where
				T: ::core::cmp::PartialEq,
				U: ::core::cmp::PartialEq
			{
				#[inline]
				fn eq(&self, __other: &Self) -> bool {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							true
							&& ::core::cmp::PartialEq::eq(__0, __other_0)
							&& ::core::cmp::PartialEq::eq(__1, __other_1),
					}
				}
			}

			impl<T, U, V> ::core::cmp::PartialOrd for Test<T, U, V>
			where
				T: ::core::cmp::PartialOrd,
				U: ::core::cmp::PartialOrd
			{
				#[inline]
				fn partial_cmp(&self, __other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
					match (self, __other) {
						(Test(ref __0, ref __1), Test(ref __other_0, ref __other_1)) =>
							match ::core::cmp::PartialOrd::partial_cmp(__0, __other_0) {
								::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(__1, __other_1) {
									::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
									__cmp => __cmp,
								},
								__cmp => __cmp,
							},
					}
				}
			}
		},
	)
}
