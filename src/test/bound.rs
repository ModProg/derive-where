use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn bound() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Clone; T)]
			struct Test<T, U>(T, core::marker::PhantomData<U>);
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
			struct Test<T, U, V>((T, U), core::marker::PhantomData<V>);
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
			struct Test<T, U>(T, core::marker::PhantomData<U>) where T: core::fmt::Debug;
		},
		quote! {
			impl<T, U> ::core::clone::Clone for Test<T, U>
			where
				T: core::fmt::Debug,
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
